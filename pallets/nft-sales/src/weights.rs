//! Autogenerated weights for pallet_nft_sales
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-02, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("development-local"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// --chain=development-local
// --steps=50
// --repeat=20
// --pallet=pallet-nft-sales
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/nft-sales/src/weights.rs
// --template=./scripts/frame-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_nft_sales.
pub trait WeightInfo {
	fn add() -> Weight;
	fn remove() -> Weight;
	fn buy() -> Weight;
}

/// Weights for pallet_nft_sales using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn add() -> Weight {
		(Weight::from_ref_time(32_000_000))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}

	fn remove() -> Weight {
		(Weight::from_ref_time(30_000_000))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}

	fn buy() -> Weight {
		(Weight::from_ref_time(50_000_000))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn add() -> Weight {
		(Weight::from_ref_time(32_000_000))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}

	fn remove() -> Weight {
		(Weight::from_ref_time(30_000_000))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}

	fn buy() -> Weight {
		(Weight::from_ref_time(50_000_000))
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}
}
