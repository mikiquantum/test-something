//! Autogenerated weights for pallet_pool_system
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-29, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("development"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=development
// --steps=50
// --repeat=20
// --pallet=pallet-pool-system
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/pools/src/weights.rs
// --template=./scripts/frame-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_pool_system.
pub trait WeightInfo {
	fn create(n: u32) -> Weight;
	fn update_no_execution(n: u32) -> Weight;
	fn update_and_execute(n: u32) -> Weight;
	fn execute_update(n: u32) -> Weight;
	fn set_max_reserve() -> Weight;
	fn close_epoch_no_orders(n: u32) -> Weight;
	fn close_epoch_no_execution(n: u32) -> Weight;
	fn close_epoch_execute(n: u32) -> Weight;
	fn submit_solution(n: u32) -> Weight;
	fn execute_epoch(n: u32) -> Weight;
}

/// Weights for pallet_pool_system using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn create(n: u32) -> Weight {
		(Weight::from_ref_time(74_485_000)) // Standard Error: 41_000
			.saturating_add((Weight::from_ref_time(299_000)).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}

	fn update_no_execution(n: u32) -> Weight {
		(Weight::from_ref_time(28_660_000)) // Standard Error: 17_000
			.saturating_add((Weight::from_ref_time(285_000)).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}

	fn update_and_execute(n: u32) -> Weight {
		(Weight::from_ref_time(47_893_000)) // Standard Error: 40_000
			.saturating_add((Weight::from_ref_time(716_000)).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}

	fn execute_update(n: u32) -> Weight {
		(Weight::from_ref_time(45_439_000)) // Standard Error: 64_000
			.saturating_add((Weight::from_ref_time(1_074_000)).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}

	fn set_max_reserve() -> Weight {
		(Weight::from_ref_time(34_009_000))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}

	fn close_epoch_no_orders(n: u32) -> Weight {
		(Weight::from_ref_time(47_513_000)) // Standard Error: 40_000
			.saturating_add((Weight::from_ref_time(7_649_000)).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n as u64)))
	}

	fn close_epoch_no_execution(n: u32) -> Weight {
		(Weight::from_ref_time(57_042_000)) // Standard Error: 78_000
			.saturating_add((Weight::from_ref_time(5_813_000)).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}

	fn close_epoch_execute(n: u32) -> Weight {
		(Weight::from_ref_time(103_700_000)) // Standard Error: 84_000
			.saturating_add((Weight::from_ref_time(9_059_000)).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n as u64)))
	}

	fn submit_solution(n: u32) -> Weight {
		(Weight::from_ref_time(39_253_000)) // Standard Error: 47_000
			.saturating_add((Weight::from_ref_time(1_711_000)).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}

	fn execute_epoch(n: u32) -> Weight {
		(Weight::from_ref_time(89_237_000)) // Standard Error: 53_000
			.saturating_add((Weight::from_ref_time(4_389_000)).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n as u64)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn create(n: u32) -> Weight {
		(Weight::from_ref_time(74_485_000)) // Standard Error: 41_000
			.saturating_add((Weight::from_ref_time(299_000)).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}

	fn update_no_execution(n: u32) -> Weight {
		(Weight::from_ref_time(28_660_000)) // Standard Error: 17_000
			.saturating_add((Weight::from_ref_time(285_000)).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}

	fn update_and_execute(n: u32) -> Weight {
		(Weight::from_ref_time(47_893_000)) // Standard Error: 40_000
			.saturating_add((Weight::from_ref_time(716_000)).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}

	fn execute_update(n: u32) -> Weight {
		(Weight::from_ref_time(45_439_000)) // Standard Error: 64_000
			.saturating_add((Weight::from_ref_time(1_074_000)).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}

	fn set_max_reserve() -> Weight {
		(Weight::from_ref_time(34_009_000))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}

	fn close_epoch_no_orders(n: u32) -> Weight {
		(Weight::from_ref_time(47_513_000)) // Standard Error: 40_000
			.saturating_add((Weight::from_ref_time(7_649_000)).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(n as u64)))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(n as u64)))
	}

	fn close_epoch_no_execution(n: u32) -> Weight {
		(Weight::from_ref_time(57_042_000)) // Standard Error: 78_000
			.saturating_add((Weight::from_ref_time(5_813_000)).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(n as u64)))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}

	fn close_epoch_execute(n: u32) -> Weight {
		(Weight::from_ref_time(103_700_000)) // Standard Error: 84_000
			.saturating_add((Weight::from_ref_time(9_059_000)).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(n as u64)))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(n as u64)))
	}

	fn submit_solution(n: u32) -> Weight {
		(Weight::from_ref_time(39_253_000)) // Standard Error: 47_000
			.saturating_add((Weight::from_ref_time(1_711_000)).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}

	fn execute_epoch(n: u32) -> Weight {
		(Weight::from_ref_time(89_237_000)) // Standard Error: 53_000
			.saturating_add((Weight::from_ref_time(4_389_000)).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(n as u64)))
	}
}
