
//! Autogenerated weights for `pallet_whitelist`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-06-19, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-172-31-15-118`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// target/release/parachain-template-node
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=benchmarking/results/results-pallet_whitelist.json
// --pallet=pallet_whitelist
// --chain=dev
// --output=benchmarking/new-benchmarks/pallet_whitelist.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for `pallet_xcm_transactor`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_xcm_transactor::WeightInfo for WeightInfo<T> {
	/// Storage: `XcmTransactor::IndexToAccount` (r:1 w:1)
	/// Proof: `XcmTransactor::IndexToAccount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn register() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `114`
		//  Estimated: `3579`
		// Minimum execution time: 9_597_000 picoseconds.
		Weight::from_parts(9_924_000, 3579)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `XcmTransactor::IndexToAccount` (r:0 w:1)
	/// Proof: `XcmTransactor::IndexToAccount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn deregister() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_707_000 picoseconds.
		Weight::from_parts(5_973_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `XcmTransactor::TransactInfoWithWeightLimit` (r:0 w:1)
	/// Proof: `XcmTransactor::TransactInfoWithWeightLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_transact_info() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_118_000 picoseconds.
		Weight::from_parts(7_356_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `XcmTransactor::TransactInfoWithWeightLimit` (r:0 w:1)
	/// Proof: `XcmTransactor::TransactInfoWithWeightLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn remove_transact_info() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_362_000 picoseconds.
		Weight::from_parts(6_609_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `XcmTransactor::DestinationAssetFeePerSecond` (r:0 w:1)
	/// Proof: `XcmTransactor::DestinationAssetFeePerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_fee_per_second() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_443_000 picoseconds.
		Weight::from_parts(6_670_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `AssetManager::AssetIdType` (r:1 w:0)
	/// Proof: `AssetManager::AssetIdType` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::IndexToAccount` (r:1 w:0)
	/// Proof: `XcmTransactor::IndexToAccount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::RelayIndices` (r:1 w:0)
	/// Proof: `XcmTransactor::RelayIndices` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::TransactInfoWithWeightLimit` (r:1 w:0)
	/// Proof: `XcmTransactor::TransactInfoWithWeightLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::DestinationAssetFeePerSecond` (r:1 w:0)
	/// Proof: `XcmTransactor::DestinationAssetFeePerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetTypeId` (r:1 w:0)
	/// Proof: `AssetManager::AssetTypeId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	fn transact_through_derivative() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `489`
		//  Estimated: `3954`
		// Minimum execution time: 30_458_000 picoseconds.
		Weight::from_parts(31_176_000, 3954)
			.saturating_add(T::DbWeight::get().reads(7_u64))
	}
	/// Storage: `AssetManager::AssetIdType` (r:1 w:0)
	/// Proof: `AssetManager::AssetIdType` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::TransactInfoWithWeightLimit` (r:1 w:0)
	/// Proof: `XcmTransactor::TransactInfoWithWeightLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::DestinationAssetFeePerSecond` (r:1 w:0)
	/// Proof: `XcmTransactor::DestinationAssetFeePerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetTypeId` (r:1 w:0)
	/// Proof: `AssetManager::AssetTypeId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	fn transact_through_sovereign() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `423`
		//  Estimated: `3888`
		// Minimum execution time: 22_713_000 picoseconds.
		Weight::from_parts(23_258_000, 3888)
			.saturating_add(T::DbWeight::get().reads(5_u64))
	}
	/// Storage: `AssetManager::AssetIdType` (r:1 w:0)
	/// Proof: `AssetManager::AssetIdType` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::TransactInfoWithWeightLimit` (r:1 w:0)
	/// Proof: `XcmTransactor::TransactInfoWithWeightLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::DestinationAssetFeePerSecond` (r:1 w:0)
	/// Proof: `XcmTransactor::DestinationAssetFeePerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	/// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn transact_through_signed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `467`
		//  Estimated: `3932`
		// Minimum execution time: 38_097_000 picoseconds.
		Weight::from_parts(38_892_000, 3932)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `XcmTransactor::RelayIndices` (r:1 w:0)
	/// Proof: `XcmTransactor::RelayIndices` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetIdType` (r:1 w:0)
	/// Proof: `AssetManager::AssetIdType` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::TransactInfoWithWeightLimit` (r:1 w:0)
	/// Proof: `XcmTransactor::TransactInfoWithWeightLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmTransactor::DestinationAssetFeePerSecond` (r:1 w:0)
	/// Proof: `XcmTransactor::DestinationAssetFeePerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	/// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn hrmp_manage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `471`
		//  Estimated: `3936`
		// Minimum execution time: 40_918_000 picoseconds.
		Weight::from_parts(42_238_000, 3936)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}