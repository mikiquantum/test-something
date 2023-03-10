// Copyright 2021 Centrifuge Foundation (centrifuge.io).
//
// This file is part of the Centrifuge chain project.
// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).
// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

use cfg_primitives::Moment;
use cfg_types::epoch::EpochState;
use codec::{Decode, Encode};
use frame_support::{
	dispatch::{DispatchError, DispatchResult},
	traits::Get,
	BoundedVec, RuntimeDebug,
};
use orml_traits::{asset_registry::AssetMetadata, Change};
use scale_info::TypeInfo;
use sp_arithmetic::traits::{BaseArithmetic, Unsigned};
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, One, Zero},
	ArithmeticError, FixedPointNumber, FixedPointOperand, TypeId,
};
use sp_std::{cmp::PartialEq, vec::Vec};

use crate::tranches::{
	EpochExecutionTranches, TrancheEssence, TrancheMetadata, TrancheSolution, TrancheUpdate,
	Tranches,
};

// The TypeId impl we derive pool-accounts from
impl<PoolId> TypeId for PoolLocator<PoolId> {
	const TYPE_ID: [u8; 4] = *b"pool";
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct ReserveDetails<Balance> {
	/// Investments will be allowed up to this amount.
	pub max: Balance,
	/// Current total amount of currency in the pool reserve.
	pub total: Balance,
	/// Current reserve that is available for originations.
	pub available: Balance,
}

impl<Balance> ReserveDetails<Balance>
where
	Balance: AtLeast32BitUnsigned + Copy + From<u64>,
{
	pub fn deposit_from_epoch<BalanceRatio, Weight, TrancheCurrency>(
		&mut self,
		epoch_tranches: &EpochExecutionTranches<Balance, BalanceRatio, Weight, TrancheCurrency>,
		solution: &[TrancheSolution],
	) -> DispatchResult
	where
		Weight: Copy + From<u128>,
		BalanceRatio: Copy,
	{
		let executed_amounts = epoch_tranches.fulfillment_cash_flows(solution)?;

		// Update the total/available reserve for the new total value of the pool
		let mut acc_investments = Balance::zero();
		let mut acc_redemptions = Balance::zero();
		for (invest, redeem) in executed_amounts.iter() {
			acc_investments = acc_investments
				.checked_add(invest)
				.ok_or(ArithmeticError::Overflow)?;
			acc_redemptions = acc_redemptions
				.checked_add(redeem)
				.ok_or(ArithmeticError::Overflow)?;
		}
		self.total = self
			.total
			.checked_add(&acc_investments)
			.ok_or(ArithmeticError::Overflow)?
			.checked_sub(&acc_redemptions)
			.ok_or(ArithmeticError::Underflow)?;

		Ok(())
	}
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct ScheduledUpdateDetails<Rate, MaxTokenNameLength, MaxTokenSymbolLength, MaxTranches>
where
	MaxTokenNameLength: Get<u32>,
	MaxTokenSymbolLength: Get<u32>,
	MaxTranches: Get<u32>,
{
	pub changes: PoolChanges<Rate, MaxTokenNameLength, MaxTokenSymbolLength, MaxTranches>,
	pub scheduled_time: Moment,
}

/// A representation of a pool identifier that can be converted to an account address
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct PoolLocator<PoolId> {
	pub pool_id: PoolId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct PoolDetails<
	CurrencyId,
	TrancheCurrency,
	EpochId,
	Balance,
	Rate,
	MetaSize,
	Weight,
	TrancheId,
	PoolId,
> where
	MetaSize: Get<u32> + Copy,
	Rate: FixedPointNumber<Inner = Balance>,
	Balance: FixedPointOperand,
{
	/// Currency that the pool is denominated in (immutable).
	pub currency: CurrencyId,
	/// List of tranches, ordered junior to senior.
	pub tranches: Tranches<Balance, Rate, Weight, TrancheCurrency, TrancheId, PoolId>,
	/// Details about the parameters of the pool.
	pub parameters: PoolParameters,
	/// Metadata that specifies the pool.
	pub metadata: Option<BoundedVec<u8, MetaSize>>,
	/// The status the pool is currently in.
	pub status: PoolStatus,
	/// Details about the epochs of the pool.
	pub epoch: EpochState<EpochId>,
	/// Details about the reserve (unused capital) in the pool.
	pub reserve: ReserveDetails<Balance>,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub enum PoolStatus {
	Open,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct PoolParameters {
	/// Minimum duration for an epoch.
	pub min_epoch_time: Moment,
	/// Maximum time between the NAV update and the epoch closing.
	pub max_nav_age: Moment,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct PoolChanges<Rate, MaxTokenNameLength, MaxTokenSymbolLength, MaxTranches>
where
	MaxTokenNameLength: Get<u32>,
	MaxTokenSymbolLength: Get<u32>,
	MaxTranches: Get<u32>,
{
	pub tranches: Change<BoundedVec<TrancheUpdate<Rate>, MaxTranches>>,
	pub tranche_metadata:
		Change<BoundedVec<TrancheMetadata<MaxTokenNameLength, MaxTokenSymbolLength>, MaxTranches>>,
	pub min_epoch_time: Change<Moment>,
	pub max_nav_age: Change<Moment>,
}

/// Information about the deposit that has been taken to create a pool
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, Default, TypeInfo)]
pub struct PoolDepositInfo<AccountId, Balance> {
	pub depositor: AccountId,
	pub deposit: Balance,
}

/// The core metadata about the pool which we can attach to an event
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct PoolEssence<
	CurrencyId,
	Balance,
	TrancheCurrency,
	Rate,
	MaxTokenNameLength,
	MaxTokenSymbolLength,
> where
	CurrencyId: Copy,
	MaxTokenNameLength: Get<u32>,
	MaxTokenSymbolLength: Get<u32>,
{
	/// Currency that the pool is denominated in (immutable).
	pub currency: CurrencyId,
	/// The maximum allowed reserve on a given pool
	pub max_reserve: Balance,
	/// Maximum time between the NAV update and the epoch closing.
	pub max_nav_age: Moment,
	/// Minimum duration for an epoch.
	pub min_epoch_time: Moment,
	/// Tranches on a pool
	pub tranches:
		Vec<TrancheEssence<TrancheCurrency, Rate, MaxTokenNameLength, MaxTokenSymbolLength>>,
}

impl<CurrencyId, TrancheCurrency, EpochId, Balance, Rate, MetaSize, Weight, TrancheId, PoolId>
	PoolDetails<
		CurrencyId,
		TrancheCurrency,
		EpochId,
		Balance,
		Rate,
		MetaSize,
		Weight,
		TrancheId,
		PoolId,
	> where
	Balance: FixedPointOperand + BaseArithmetic + Unsigned + From<u64>,
	CurrencyId: Copy,
	EpochId: BaseArithmetic,
	MetaSize: Get<u32> + Copy,
	PoolId: Copy + Encode,
	Rate: FixedPointNumber<Inner = Balance>,
	TrancheCurrency: Copy + cfg_traits::TrancheCurrency<PoolId, TrancheId>,
	TrancheId: Clone + From<[u8; 16]> + PartialEq,
	Weight: Copy + From<u128>,
{
	pub fn start_next_epoch(&mut self, now: Moment) -> DispatchResult {
		self.epoch.current += One::one();
		self.epoch.last_closed = now;
		// TODO: Remove and set state rather to EpochClosing or similar
		// Set available reserve to 0 to disable originations while the epoch is closed but not executed
		self.reserve.available = Zero::zero();

		Ok(())
	}

	pub fn execute_previous_epoch(&mut self) -> DispatchResult {
		self.reserve.available = self.reserve.total;
		self.epoch.last_executed += One::one();
		Ok(())
	}

	pub fn essence<AssetRegistry, AssetId, MaxTokenNameLength, MaxTokenSymbolLength>(
		&self,
	) -> Result<
		PoolEssence<
			CurrencyId,
			Balance,
			TrancheCurrency,
			Rate,
			MaxTokenNameLength,
			MaxTokenSymbolLength,
		>,
		DispatchError,
	>
	where
		AssetRegistry: orml_traits::asset_registry::Inspect,
		<AssetRegistry as orml_traits::asset_registry::Inspect>::AssetId: From<CurrencyId>,
		MaxTokenNameLength: Get<u32>,
		MaxTokenSymbolLength: Get<u32>,
	{
		let mut tranches: Vec<
			TrancheEssence<TrancheCurrency, Rate, MaxTokenNameLength, MaxTokenSymbolLength>,
		> = Vec::new();

		for tranche in self.tranches.residual_top_slice().iter() {
			let metadata = AssetRegistry::metadata(&self.currency.into())
				.ok_or(AssetMetadata {
					decimals: 0,
					name: Vec::new(),
					symbol: Vec::new(),
					existential_deposit: (),
					location: None,
					additional: (),
				})
				.unwrap();

			tranches.push(TrancheEssence {
				currency: tranche.currency.into(),
				ty: tranche.tranche_type.into(),
				metadata: TrancheMetadata {
					token_name: BoundedVec::try_from(metadata.name)
						.unwrap_or(BoundedVec::default()),
					token_symbol: BoundedVec::try_from(metadata.symbol)
						.unwrap_or(BoundedVec::default()),
				},
			});
		}

		Ok(PoolEssence {
			currency: self.currency,
			max_reserve: self.reserve.max.into(),
			max_nav_age: self.parameters.max_nav_age,
			min_epoch_time: self.parameters.min_epoch_time,
			tranches,
		})
	}
}
