//! Autogenerated weights for pallet_preimage
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
// --pallet=pallet_preimage
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_preimage.rs
// --template=./scripts/runtime-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use pallet_preimage::weights::WeightInfo;
use sp_std::marker::PhantomData;

/// Weights for pallet_preimage using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn note_preimage(s: u32) -> Weight {
		Weight::from_ref_time(0) // Standard Error: 0
			.saturating_add(Weight::from_ref_time(3_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}

	fn note_requested_preimage(s: u32) -> Weight {
		Weight::from_ref_time(0) // Standard Error: 0
			.saturating_add(Weight::from_ref_time(3_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn note_no_deposit_preimage(s: u32) -> Weight {
		Weight::from_ref_time(0) // Standard Error: 0
			.saturating_add(Weight::from_ref_time(3_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn unnote_preimage() -> Weight {
		Weight::from_ref_time(137_640_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}

	fn unnote_no_deposit_preimage() -> Weight {
		Weight::from_ref_time(105_798_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}

	fn request_preimage() -> Weight {
		Weight::from_ref_time(134_807_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn request_no_deposit_preimage() -> Weight {
		Weight::from_ref_time(101_782_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn request_unnoted_preimage() -> Weight {
		Weight::from_ref_time(66_956_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn request_requested_preimage() -> Weight {
		Weight::from_ref_time(30_877_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn unrequest_preimage() -> Weight {
		Weight::from_ref_time(104_814_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}

	fn unrequest_unnoted_preimage() -> Weight {
		Weight::from_ref_time(61_925_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}

	fn unrequest_multi_referenced_preimage() -> Weight {
		Weight::from_ref_time(29_312_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
