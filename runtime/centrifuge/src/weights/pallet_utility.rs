//! Autogenerated weights for pallet_utility
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-02, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("centrifuge-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=centrifuge-dev
// --steps=50
// --repeat=20
// --pallet=pallet_utility
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_utility.rs
// --template=./scripts/runtime-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use pallet_utility::weights::WeightInfo;
use sp_std::marker::PhantomData;

/// Weights for pallet_utility using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn batch(c: u32) -> Weight {
		Weight::from_ref_time(67_513_000) // Standard Error: 21_000
			.saturating_add(Weight::from_ref_time(9_255_000).saturating_mul(c as u64))
	}

	fn as_derivative() -> Weight {
		Weight::from_ref_time(12_924_000)
	}

	fn batch_all(c: u32) -> Weight {
		Weight::from_ref_time(64_070_000) // Standard Error: 18_000
			.saturating_add(Weight::from_ref_time(9_663_000).saturating_mul(c as u64))
	}

	fn dispatch_as() -> Weight {
		Weight::from_ref_time(29_522_000)
	}

	fn force_batch(c: u32) -> Weight {
		Weight::from_ref_time(69_875_000) // Standard Error: 13_000
			.saturating_add(Weight::from_ref_time(9_252_000).saturating_mul(c as u64))
	}
}
