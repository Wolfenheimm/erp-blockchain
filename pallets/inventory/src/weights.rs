
//! Autogenerated weights for `pallet_inventory`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 43.0.0
//! DATE: 2024-11-01, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Leviathan.local`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: `1024`

// Executed Command:
// frame-omni-bencher
// v1
// benchmark
// pallet
// --runtime
// target/production/wbuild/erp-blockchain-runtime/erp_blockchain_runtime.compact.compressed.wasm
// --pallet
// pallet-inventory
// --extrinsic
// *
// --genesis-builder-preset=production
// --output
// ./pallets/inventory/src/weights.rs
// --template
// .maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_inventory`.
pub trait WeightInfo {
	fn inventory_insertion() -> Weight;
}

/// Weights for `pallet_inventory` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Inventory::Inventory` (r:1 w:1)
	/// Proof: `Inventory::Inventory` (`max_values`: None, `max_size`: Some(4983), added: 7458, mode: `MaxEncodedLen`)
	fn inventory_insertion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `8448`
		// Minimum execution time: 9_000_000 picoseconds.
		Weight::from_parts(9_000_000, 8448)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Inventory::Inventory` (r:1 w:1)
	/// Proof: `Inventory::Inventory` (`max_values`: None, `max_size`: Some(4983), added: 7458, mode: `MaxEncodedLen`)
	fn inventory_insertion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `8448`
		// Minimum execution time: 9_000_000 picoseconds.
		Weight::from_parts(9_000_000, 8448)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}