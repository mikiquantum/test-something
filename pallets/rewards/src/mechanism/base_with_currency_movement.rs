use cfg_traits::ops::ensure::{
	EnsureAdd, EnsureAddAssign, EnsureFixedPointNumber, EnsureFrom, EnsureInto, EnsureSub,
	EnsureSubAssign,
};
use frame_support::{pallet_prelude::*, traits::tokens};
use num_traits::Signed;
use sp_runtime::{traits::Zero, ArithmeticError, FixedPointNumber, FixedPointOperand};

use super::{base, MoveCurrencyError, RewardMechanism};

/// Type that contains the stake properties of an account
#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug, Default)]
#[cfg_attr(test, derive(PartialEq, Clone))]
pub struct Account<Balance, IBalance> {
	pub base: base::Account<Balance, IBalance>,
	pub last_currency_movement: u32,
}

impl<Balance, IBalance> Account<Balance, IBalance>
where
	Balance: FixedPointOperand + EnsureAdd + EnsureSub + TryFrom<IBalance> + Copy,
	IBalance: FixedPointOperand + TryFrom<Balance> + EnsureAdd + EnsureSub + Copy,
{
	pub fn get_tally_from_rpt_changes<Rate: FixedPointNumber>(
		&self,
		rpt_changes: &[Rate],
	) -> Result<IBalance, ArithmeticError> {
		let rpt_to_apply = &rpt_changes[self.last_currency_movement as usize..]
			.iter()
			.try_fold(Rate::zero(), |a, b| a.ensure_add(*b))?;

		rpt_to_apply.ensure_mul_int(IBalance::ensure_from(self.base.stake)?)
	}

	pub fn apply_rpt_changes<Rate: FixedPointNumber>(
		&mut self,
		rpt_changes: &[Rate],
	) -> Result<(), ArithmeticError> {
		let tally_to_apply = self.get_tally_from_rpt_changes(rpt_changes)?;

		self.base.reward_tally.ensure_add_assign(tally_to_apply)?;
		self.last_currency_movement = rpt_changes.len() as u32;

		Ok(())
	}
}

/// Type that contains the stake properties of stake class
#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug)]
#[cfg_attr(test, derive(PartialEq, Clone))]
pub struct Currency<Balance, Rate, MaxMovements: Get<u32>> {
	pub total_stake: Balance,
	pub rpt_changes: BoundedVec<Rate, MaxMovements>,
}

impl<Balance, Rate, MaxMovements> Default for Currency<Balance, Rate, MaxMovements>
where
	Balance: Zero,
	Rate: Zero,
	MaxMovements: Get<u32>,
{
	fn default() -> Self {
		Self {
			total_stake: Zero::zero(),
			rpt_changes: BoundedVec::default(),
		}
	}
}

pub struct Mechanism<Balance, IBalance, Rate, MaxCurrencyMovements>(
	sp_std::marker::PhantomData<(Balance, IBalance, Rate, MaxCurrencyMovements)>,
);

impl<Balance, IBalance, Rate, MaxCurrencyMovements> RewardMechanism
	for Mechanism<Balance, IBalance, Rate, MaxCurrencyMovements>
where
	Balance: tokens::Balance + FixedPointOperand + TryFrom<IBalance>,
	IBalance: FixedPointOperand
		+ TryFrom<Balance>
		+ EnsureAdd
		+ EnsureSub
		+ Copy
		+ Signed
		+ sp_std::fmt::Debug,
	Rate: EnsureFixedPointNumber,
	MaxCurrencyMovements: Get<u32>,
	<Rate as FixedPointNumber>::Inner: Signed,
{
	type Account = Account<Self::Balance, IBalance>;
	type Balance = Balance;
	type Currency = Currency<Balance, Rate, MaxCurrencyMovements>;
	type DistributionId = ();
	type Group = base::Group<Balance, Rate>;
	type MaxCurrencyMovements = MaxCurrencyMovements;

	fn reward_group(
		group: &mut Self::Group,
		amount: Self::Balance,
		distribution_id: Self::DistributionId,
	) -> Result<(), ArithmeticError> {
		base::Mechanism::<Balance, IBalance, Rate>::reward_group(group, amount, distribution_id)
	}

	fn deposit_stake(
		account: &mut Self::Account,
		currency: &mut Self::Currency,
		group: &mut Self::Group,
		amount: Self::Balance,
	) -> Result<(), ArithmeticError> {
		account.apply_rpt_changes(&currency.rpt_changes)?;

		base::Mechanism::<Balance, IBalance, Rate>::deposit_stake(
			&mut account.base,
			&mut (),
			group,
			amount,
		)?;

		currency.total_stake.ensure_add_assign(amount)
	}

	fn withdraw_stake(
		account: &mut Self::Account,
		currency: &mut Self::Currency,
		group: &mut Self::Group,
		amount: Self::Balance,
	) -> Result<(), ArithmeticError> {
		account.apply_rpt_changes(&currency.rpt_changes)?;

		base::Mechanism::<Balance, IBalance, Rate>::withdraw_stake(
			&mut account.base,
			&mut (),
			group,
			amount,
		)?;

		currency.total_stake.ensure_sub_assign(amount)
	}

	fn compute_reward(
		account: &Self::Account,
		currency: &Self::Currency,
		group: &Self::Group,
	) -> Result<Self::Balance, ArithmeticError> {
		let extra_tally = account.get_tally_from_rpt_changes(&currency.rpt_changes)?;

		let base_reward =
			base::Mechanism::<Balance, IBalance, Rate>::compute_reward(&account.base, &(), group)?;

		IBalance::ensure_from(base_reward)?
			.ensure_sub(extra_tally)?
			.ensure_into()
	}

	fn claim_reward(
		account: &mut Self::Account,
		currency: &Self::Currency,
		group: &Self::Group,
	) -> Result<Self::Balance, ArithmeticError> {
		account.apply_rpt_changes(&currency.rpt_changes)?;

		base::Mechanism::<Balance, IBalance, Rate>::claim_reward(&mut account.base, &(), group)
	}

	fn move_currency(
		currency: &mut Self::Currency,
		prev_group: &mut Self::Group,
		next_group: &mut Self::Group,
	) -> Result<(), MoveCurrencyError> {
		let rpt_change = next_group
			.reward_per_token
			.ensure_sub(prev_group.reward_per_token)?;

		currency
			.rpt_changes
			.try_push(rpt_change)
			.map_err(|_| MoveCurrencyError::MaxMovements)?;

		prev_group
			.total_stake
			.ensure_sub_assign(currency.total_stake)?;

		next_group
			.total_stake
			.ensure_add_assign(currency.total_stake)?;

		Ok(())
	}

	fn account_stake(account: &Self::Account) -> Self::Balance {
		account.base.stake
	}

	fn group_stake(group: &Self::Group) -> Self::Balance {
		group.total_stake
	}
}

#[cfg(test)]
mod test {
	use frame_support::bounded_vec;
	use sp_runtime::FixedI64;

	use super::*;

	type Balance = u64;
	type IBalance = i64;
	type Rate = FixedI64;

	type TestMechanism = Mechanism<Balance, IBalance, Rate, MaxCurrencyMovements>;

	frame_support::parameter_types! {
		#[derive(scale_info::TypeInfo, PartialEq, Clone, Debug)]
		pub const MaxCurrencyMovements: u32 = 4;
	}

	const AMOUNT: u64 = crate::mechanism::test::AMOUNT;

	pub mod initial {
		use super::*;

		lazy_static::lazy_static! {
			pub static ref GROUP: base::Group<Balance, Rate> = base::test::initial::GROUP.clone();

			pub static ref NEXT_GROUP: base::Group<Balance, Rate> = base::test::initial::NEXT_GROUP.clone();

			pub static ref ACCOUNT: Account<Balance, IBalance> = Account {
				base: base::test::initial::ACCOUNT.clone(),
				last_currency_movement: 1,
			};

			pub static ref CURRENCY: Currency<Balance, Rate, MaxCurrencyMovements> = Currency {
				total_stake: 200,
				rpt_changes: bounded_vec![
					Rate::from(5),
					Rate::from(-2),
					Rate::from(3),
				],
			};
		}
	}

	pub mod expectation {
		use super::{initial::*, *};

		lazy_static::lazy_static! {
			pub static ref DEPOSIT_STAKE__GROUP: base::Group<Balance, Rate> =
				base::test::expectation::DEPOSIT_STAKE__GROUP.clone();

			pub static ref DEPOSIT_STAKE__ACCOUNT: Account<Balance, IBalance> = Account {
				base: base::Account {
					stake: base::test::expectation::DEPOSIT_STAKE__ACCOUNT.stake,
					reward_tally: base::test::expectation::DEPOSIT_STAKE__ACCOUNT.reward_tally
						+ *RPT_CHANGE_TALLY,
				},
				last_currency_movement: CURRENCY.rpt_changes.len() as u32,
			};

			pub static ref DEPOSIT_STAKE__CURRENCY: Currency<Balance, Rate, MaxCurrencyMovements> = Currency {
				total_stake: CURRENCY.total_stake + AMOUNT,
				rpt_changes: CURRENCY.rpt_changes.clone(),
			};

			pub static ref WITHDRAW_STAKE__GROUP: base::Group<Balance, Rate> =
				base::test::expectation::WITHDRAW_STAKE__GROUP.clone();

			pub static ref WITHDRAW_STAKE__ACCOUNT: Account<Balance, IBalance> = Account {
				base: base::Account {
					stake: base::test::expectation::WITHDRAW_STAKE__ACCOUNT.stake,
					reward_tally: base::test::expectation::WITHDRAW_STAKE__ACCOUNT.reward_tally
						+ *RPT_CHANGE_TALLY,
				},
				last_currency_movement: CURRENCY.rpt_changes.len() as u32,
			};

			pub static ref WITHDRAW_STAKE__CURRENCY: Currency<Balance, Rate, MaxCurrencyMovements> = Currency {
				total_stake: CURRENCY.total_stake - AMOUNT,
				rpt_changes: CURRENCY.rpt_changes.clone(),
			};

			pub static ref CLAIM__ACCOUNT: Account<Balance, IBalance> = Account {
				base: base::test::expectation::CLAIM__ACCOUNT.clone(),
				last_currency_movement: CURRENCY.rpt_changes.len() as u32,
			};

			pub static ref CLAIM__REWARD: u64 =
				(*base::test::expectation::CLAIM__REWARD as i64 - *RPT_CHANGE_TALLY)as u64;

			pub static ref MOVE_CURRENCY__CURRENCY: Currency<Balance, Rate, MaxCurrencyMovements> = Currency {
				total_stake: CURRENCY.total_stake,
				rpt_changes: bounded_vec![
					CURRENCY.rpt_changes[0],
					CURRENCY.rpt_changes[1],
					CURRENCY.rpt_changes[2],
					base::test::initial::NEXT_GROUP.reward_per_token - base::test::initial::GROUP.reward_per_token,
				],
			};
			pub static ref MOVE_CURRENCY__GROUP_PREV: base::Group<Balance, Rate> = base::Group {
				total_stake: base::test::initial::GROUP.total_stake - CURRENCY.total_stake,
				reward_per_token: base::test::initial::GROUP.reward_per_token,
			};

			pub static ref MOVE_CURRENCY__GROUP_NEXT: base::Group<Balance, Rate> = base::Group {
				total_stake: base::test::initial::NEXT_GROUP.total_stake + CURRENCY.total_stake,
				reward_per_token: base::test::initial::NEXT_GROUP.reward_per_token,
			};

			static ref RPT_CHANGE_TALLY: i64 =
				(CURRENCY.rpt_changes[1] + CURRENCY.rpt_changes[2]).saturating_mul_int(ACCOUNT.base.stake as i64);
		}
	}

	crate::mechanism_deposit_stake_test_impl!(TestMechanism, initial, expectation);
	crate::mechanism_withdraw_stake_test_impl!(TestMechanism, initial, expectation);
	crate::mechanism_claim_reward_test_impl!(TestMechanism, initial, expectation);
	crate::mechanism_move_currency_test_impl!(TestMechanism, initial, expectation);
}
