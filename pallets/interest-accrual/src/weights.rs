//! Autogenerated weights for pallet_interest_accrual
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-08-01, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("development"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=development
// --steps=50
// --repeat=20
// --pallet=pallet-interest-accrual
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/interest-accrual/src/weights.rs
// --template=./scripts/frame-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_interest_accrual.
pub trait WeightInfo {
	fn calculate_accumulated_rate(n: u32) -> Weight;
}

/// Weights for pallet_interest_accrual using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn calculate_accumulated_rate(n: u32) -> Weight {
		(Weight::from_ref_time(0)) // Standard Error: 8_000
			.saturating_add((Weight::from_ref_time(9_747_000)).saturating_mul(n as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn calculate_accumulated_rate(n: u32) -> Weight {
		(Weight::from_ref_time(0)) // Standard Error: 8_000
			.saturating_add((Weight::from_ref_time(9_747_000)).saturating_mul(n as u64))
	}
}
