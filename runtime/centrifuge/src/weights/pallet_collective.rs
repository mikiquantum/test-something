//! Autogenerated weights for pallet_collective
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
// --pallet=pallet_collective
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_collective.rs
// --template=./scripts/runtime-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weights for pallet_collective using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
	fn set_members(m: u32, n: u32, p: u32) -> Weight {
		Weight::from_ref_time(0) // Standard Error: 16_000
			.saturating_add(Weight::from_ref_time(17_858_000).saturating_mul(m as u64)) // Standard Error: 16_000
			.saturating_add(Weight::from_ref_time(18_000).saturating_mul(n as u64)) // Standard Error: 16_000
			.saturating_add(Weight::from_ref_time(25_232_000).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(p as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(p as u64)))
	}

	fn execute(b: u32, m: u32) -> Weight {
		Weight::from_ref_time(36_239_000) // Standard Error: 0
			.saturating_add(Weight::from_ref_time(6_000).saturating_mul(b as u64)) // Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(73_000).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}

	fn propose_execute(b: u32, m: u32) -> Weight {
		Weight::from_ref_time(45_624_000) // Standard Error: 0
			.saturating_add(Weight::from_ref_time(1_000).saturating_mul(b as u64)) // Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(115_000).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
	}

	fn propose_proposed(b: u32, m: u32, p: u32) -> Weight {
		Weight::from_ref_time(53_192_000) // Standard Error: 0
			.saturating_add(Weight::from_ref_time(16_000).saturating_mul(b as u64)) // Standard Error: 4_000
			.saturating_add(Weight::from_ref_time(60_000).saturating_mul(m as u64)) // Standard Error: 4_000
			.saturating_add(Weight::from_ref_time(553_000).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}

	fn vote(m: u32) -> Weight {
		Weight::from_ref_time(76_580_000) // Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(223_000).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn close_early_disapproved(m: u32, p: u32) -> Weight {
		Weight::from_ref_time(66_943_000) // Standard Error: 4_000
			.saturating_add(Weight::from_ref_time(140_000).saturating_mul(m as u64)) // Standard Error: 4_000
			.saturating_add(Weight::from_ref_time(444_000).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}

	fn close_early_approved(b: u32, m: u32, p: u32) -> Weight {
		Weight::from_ref_time(68_260_000) // Standard Error: 0
			.saturating_add(Weight::from_ref_time(15_000).saturating_mul(b as u64)) // Standard Error: 4_000
			.saturating_add(Weight::from_ref_time(194_000).saturating_mul(m as u64)) // Standard Error: 4_000
			.saturating_add(Weight::from_ref_time(616_000).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}

	fn close_disapproved(m: u32, p: u32) -> Weight {
		Weight::from_ref_time(74_363_000) // Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(130_000).saturating_mul(m as u64)) // Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(451_000).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}

	fn close_approved(b: u32, m: u32, p: u32) -> Weight {
		Weight::from_ref_time(73_899_000) // Standard Error: 0
			.saturating_add(Weight::from_ref_time(16_000).saturating_mul(b as u64)) // Standard Error: 4_000
			.saturating_add(Weight::from_ref_time(198_000).saturating_mul(m as u64)) // Standard Error: 4_000
			.saturating_add(Weight::from_ref_time(598_000).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}

	fn disapprove_proposal(p: u32) -> Weight {
		Weight::from_ref_time(43_815_000) // Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(528_000).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
}
