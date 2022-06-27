// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-27, STEPS: `1`, REPEAT: 1, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/joystream-node
// benchmark
// pallet
// --pallet=pallet_staking
// --extrinsic=*
// --chain=dev
// --steps=1
// --repeat=1
// --execution=wasm
// --template=./scripts/../devops/frame-weight-template.hbs
// --output=.

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

pub use pallet_staking::weights::WeightInfo;

/// Weights for pallet_staking using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca3ed14b45ed20d054f05e37e2542cfe70] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca0b6a45321efae92aea15e0740ec7afe7] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaac0a2cbf8e355f5ea6cb2de8727bfb0c] (r:1 w:0)
	// Storage: unknown [0xc2261276cc9d1f8598ea4b6a74b15c2f218f26c73add634897550b4003b26bc6] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca9220e172bed316605f73f1ff7b4ade98] (r:0 w:1)
	fn bond() -> Weight {
		(48_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca3ed14b45ed20d054f05e37e2542cfe70] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4] (r:1 w:1)
	// Storage: unknown [0xc2261276cc9d1f8598ea4b6a74b15c2f218f26c73add634897550b4003b26bc6] (r:1 w:1)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685e62556a85fcb7c61b2c6c750924846b15] (r:3 w:3)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685ee5c03730c8f59f00941607850b6633d8] (r:2 w:2)
	fn bond_extra() -> Weight {
		(69_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca9c6a637f62ae2af1c7e31eed7e96be04] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaed441ceb81326c56263efbb60c95c2e4] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca0b6a45321efae92aea15e0740ec7afe7] (r:1 w:0)
	// Storage: unknown [0xc2261276cc9d1f8598ea4b6a74b15c2f218f26c73add634897550b4003b26bc6] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685e62556a85fcb7c61b2c6c750924846b15] (r:3 w:3)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca3ed14b45ed20d054f05e37e2542cfe70] (r:1 w:0)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685ee5c03730c8f59f00941607850b6633d8] (r:2 w:2)
	fn unbond() -> Weight {
		(82_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca0b6a45321efae92aea15e0740ec7afe7] (r:1 w:0)
	// Storage: unknown [0xc2261276cc9d1f8598ea4b6a74b15c2f218f26c73add634897550b4003b26bc6] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	fn withdraw_unbonded_update(_s: u32, ) -> Weight {
		(35_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca0b6a45321efae92aea15e0740ec7afe7] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca3ed14b45ed20d054f05e37e2542cfe70] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaab6a212bc08a5603828f33f90ec4a139] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca88dcde934c658227ee1dfafcd6e16903] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca9c6a637f62ae2af1c7e31eed7e96be04] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaf99b25852d3d69419882da651375cdb3] (r:1 w:1)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685e62556a85fcb7c61b2c6c750924846b15] (r:2 w:2)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685ee5c03730c8f59f00941607850b6633d8] (r:1 w:1)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685e7a6dc62e324093ba1331bf49fdb2f24a] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	// Storage: unknown [0xc2261276cc9d1f8598ea4b6a74b15c2f218f26c73add634897550b4003b26bc6] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca9220e172bed316605f73f1ff7b4ade98] (r:0 w:1)
	fn withdraw_unbonded_kill(_s: u32, ) -> Weight {
		(65_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(13 as Weight))
			.saturating_add(T::DbWeight::get().writes(11 as Weight))
	}
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca666fdcbb473985b3ac933d13f4acff8d] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca58b0c9c1fa6cc13759ead9b42db9eebe] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca88dcde934c658227ee1dfafcd6e16903] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca98c2640cda6c0d801194a8a61c699224] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca9c6a637f62ae2af1c7e31eed7e96be04] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca3ed14b45ed20d054f05e37e2542cfe70] (r:1 w:0)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685e62556a85fcb7c61b2c6c750924846b15] (r:2 w:2)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685ee5c03730c8f59f00941607850b6633d8] (r:1 w:1)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685e7a6dc62e324093ba1331bf49fdb2f24a] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca6ddc7809c6da9bb6093ee22e0fda4ba8] (r:1 w:1)
	fn validate() -> Weight {
		(62_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca9c6a637f62ae2af1c7e31eed7e96be04] (r:1 w:1)
	fn kick(_k: u32, ) -> Weight {
		(1_070_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(129 as Weight))
			.saturating_add(T::DbWeight::get().writes(128 as Weight))
	}
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaed441ceb81326c56263efbb60c95c2e4] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca9c6a637f62ae2af1c7e31eed7e96be04] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcad642c00af119adf30dc11d32e9f0886d] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca88dcde934c658227ee1dfafcd6e16903] (r:2 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca0b6a45321efae92aea15e0740ec7afe7] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca3ed14b45ed20d054f05e37e2542cfe70] (r:1 w:0)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685e62556a85fcb7c61b2c6c750924846b15] (r:2 w:2)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685ee5c03730c8f59f00941607850b6633d8] (r:1 w:1)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685e7a6dc62e324093ba1331bf49fdb2f24a] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaf99b25852d3d69419882da651375cdb3] (r:1 w:1)
	fn nominate(_n: u32, ) -> Weight {
		(115_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(28 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca88dcde934c658227ee1dfafcd6e16903] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca9c6a637f62ae2af1c7e31eed7e96be04] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaf99b25852d3d69419882da651375cdb3] (r:1 w:1)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685e62556a85fcb7c61b2c6c750924846b15] (r:2 w:2)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685ee5c03730c8f59f00941607850b6633d8] (r:1 w:1)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685e7a6dc62e324093ba1331bf49fdb2f24a] (r:1 w:1)
	fn chill() -> Weight {
		(46_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca9220e172bed316605f73f1ff7b4ade98] (r:0 w:1)
	fn set_payee() -> Weight {
		(7_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca3ed14b45ed20d054f05e37e2542cfe70] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4] (r:2 w:2)
	fn set_controller() -> Weight {
		(15_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca138e71612491192d68deab7e6f563fe1] (r:0 w:1)
	fn set_validator_count() -> Weight {
		(2_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaf7dad0317324aecae8744b87fc95f2f3] (r:0 w:1)
	fn force_no_eras() -> Weight {
		(2_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaf7dad0317324aecae8744b87fc95f2f3] (r:0 w:1)
	fn force_new_era() -> Weight {
		(1_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaf7dad0317324aecae8744b87fc95f2f3] (r:0 w:1)
	fn force_new_era_always() -> Weight {
		(1_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca5579297f4dfb9609e7e4c2ebab9ce40a] (r:0 w:1)
	fn set_invulnerables(_v: u32, ) -> Weight {
		(10_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca3ed14b45ed20d054f05e37e2542cfe70] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaab6a212bc08a5603828f33f90ec4a139] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca88dcde934c658227ee1dfafcd6e16903] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca9c6a637f62ae2af1c7e31eed7e96be04] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaf99b25852d3d69419882da651375cdb3] (r:1 w:1)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685e62556a85fcb7c61b2c6c750924846b15] (r:2 w:2)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685ee5c03730c8f59f00941607850b6633d8] (r:1 w:1)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685e7a6dc62e324093ba1331bf49fdb2f24a] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	// Storage: unknown [0xc2261276cc9d1f8598ea4b6a74b15c2f218f26c73add634897550b4003b26bc6] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4] (r:0 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca9220e172bed316605f73f1ff7b4ade98] (r:0 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcae62f6f797ebe9138dfced942977fea50] (r:0 w:100)
	fn force_unstake(_s: u32, ) -> Weight {
		(144_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().writes(112 as Weight))
	}
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca042824170a5db4381fe3395039cabd24] (r:1 w:1)
	fn cancel_deferred_slash(_s: u32, ) -> Weight {
		(3_597_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca0b6a45321efae92aea15e0740ec7afe7] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaac0a2cbf8e355f5ea6cb2de8727bfb0c] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca7e6ed2ee507c7b4441d59e4ded44b8a2] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca3ed14b45ed20d054f05e37e2542cfe70] (r:2 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca42982b9d6c7acc99faa9094c912372c2] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca80cc6574281671b299c1727d7ac68cab] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca682db92dde20a10d96d00ff0e9e221c0] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca9220e172bed316605f73f1ff7b4ade98] (r:2 w:0)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:2 w:2)
	fn payout_stakers_dead_controller(_n: u32, ) -> Weight {
		(6_749_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(778 as Weight))
			.saturating_add(T::DbWeight::get().writes(258 as Weight))
	}
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca0b6a45321efae92aea15e0740ec7afe7] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaac0a2cbf8e355f5ea6cb2de8727bfb0c] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca7e6ed2ee507c7b4441d59e4ded44b8a2] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca3ed14b45ed20d054f05e37e2542cfe70] (r:2 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4] (r:2 w:2)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca42982b9d6c7acc99faa9094c912372c2] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca80cc6574281671b299c1727d7ac68cab] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca682db92dde20a10d96d00ff0e9e221c0] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca9220e172bed316605f73f1ff7b4ade98] (r:2 w:0)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:2 w:2)
	// Storage: unknown [0xc2261276cc9d1f8598ea4b6a74b15c2f218f26c73add634897550b4003b26bc6] (r:2 w:2)
	fn payout_stakers_alive_staked(_n: u32, ) -> Weight {
		(8_920_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1291 as Weight))
			.saturating_add(T::DbWeight::get().writes(771 as Weight))
	}
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4] (r:1 w:1)
	// Storage: unknown [0xc2261276cc9d1f8598ea4b6a74b15c2f218f26c73add634897550b4003b26bc6] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685e62556a85fcb7c61b2c6c750924846b15] (r:3 w:3)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca3ed14b45ed20d054f05e37e2542cfe70] (r:1 w:0)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685ee5c03730c8f59f00941607850b6633d8] (r:2 w:2)
	fn rebond(_l: u32, ) -> Weight {
		(71_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca0b6a45321efae92aea15e0740ec7afe7] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaac0a2cbf8e355f5ea6cb2de8727bfb0c] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca42982b9d6c7acc99faa9094c912372c2] (r:0 w:2)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca682db92dde20a10d96d00ff0e9e221c0] (r:0 w:2)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca7e6ed2ee507c7b4441d59e4ded44b8a2] (r:0 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca80cc6574281671b299c1727d7ac68cab] (r:0 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca8bde0a0ea8864605e3b68ed9cb2da01b] (r:0 w:2)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaa141c4fe67c2d11f4a10c6aca7a79a04] (r:0 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaad811cd65a470ddc5f1d628ff0550982] (r:0 w:1)
	fn set_history_depth(_e: u32, ) -> Weight {
		(2_300_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(704 as Weight))
	}
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca3ed14b45ed20d054f05e37e2542cfe70] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaab6a212bc08a5603828f33f90ec4a139] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca88dcde934c658227ee1dfafcd6e16903] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca9c6a637f62ae2af1c7e31eed7e96be04] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaf99b25852d3d69419882da651375cdb3] (r:1 w:1)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685e62556a85fcb7c61b2c6c750924846b15] (r:2 w:2)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685ee5c03730c8f59f00941607850b6633d8] (r:1 w:1)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685e7a6dc62e324093ba1331bf49fdb2f24a] (r:1 w:1)
	// Storage: unknown [0xc2261276cc9d1f8598ea4b6a74b15c2f218f26c73add634897550b4003b26bc6] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca9220e172bed316605f73f1ff7b4ade98] (r:0 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcae62f6f797ebe9138dfced942977fea50] (r:0 w:1)
	fn reap_stash(_s: u32, ) -> Weight {
		(153_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(112 as Weight))
	}
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685e7a6dc62e324093ba1331bf49fdb2f24a] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaab6a212bc08a5603828f33f90ec4a139] (r:1 w:0)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685ee5c03730c8f59f00941607850b6633d8] (r:200 w:0)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685e62556a85fcb7c61b2c6c750924846b15] (r:101 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca9c6a637f62ae2af1c7e31eed7e96be04] (r:101 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca88dcde934c658227ee1dfafcd6e16903] (r:2 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca3ed14b45ed20d054f05e37e2542cfe70] (r:101 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4] (r:101 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca6ddc7809c6da9bb6093ee22e0fda4ba8] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca138e71612491192d68deab7e6f563fe1] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcab49a2738eeb30896aacb8b3fb46471bd] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca0b6a45321efae92aea15e0740ec7afe7] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaac0a2cbf8e355f5ea6cb2de8727bfb0c] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca42982b9d6c7acc99faa9094c912372c2] (r:0 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca682db92dde20a10d96d00ff0e9e221c0] (r:0 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca8bde0a0ea8864605e3b68ed9cb2da01b] (r:0 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaa141c4fe67c2d11f4a10c6aca7a79a04] (r:0 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaad811cd65a470ddc5f1d628ff0550982] (r:0 w:1)
	fn new_era(v: u32, n: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 7_120_000
			.saturating_add((101_556_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 647_000
			.saturating_add((24_051_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(208 as Weight))
			.saturating_add(T::DbWeight::get().reads((5 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().reads((4 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685e7a6dc62e324093ba1331bf49fdb2f24a] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaab6a212bc08a5603828f33f90ec4a139] (r:21 w:0)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685ee5c03730c8f59f00941607850b6633d8] (r:200 w:0)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685e62556a85fcb7c61b2c6c750924846b15] (r:1500 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca9c6a637f62ae2af1c7e31eed7e96be04] (r:1500 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca88dcde934c658227ee1dfafcd6e16903] (r:500 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca3ed14b45ed20d054f05e37e2542cfe70] (r:1500 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4] (r:1500 w:0)
	fn get_npos_voters(v: u32, n: u32, s: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 1_025_000
			.saturating_add((25_337_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 1_025_000
			.saturating_add((23_455_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(202 as Weight))
			.saturating_add(T::DbWeight::get().reads((5 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().reads((4 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca88dcde934c658227ee1dfafcd6e16903] (r:501 w:0)
	fn get_npos_targets(_v: u32, ) -> Weight {
		(9_250_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1001 as Weight))
	}
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca58b0c9c1fa6cc13759ead9b42db9eebe] (r:0 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca666fdcbb473985b3ac933d13f4acff8d] (r:0 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca98c2640cda6c0d801194a8a61c699224] (r:0 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcacddc49c5f30807d474a09d70fed8a569] (r:0 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcad642c00af119adf30dc11d32e9f0886d] (r:0 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaed441ceb81326c56263efbb60c95c2e4] (r:0 w:1)
	fn set_staking_configs_all_set() -> Weight {
		(4_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca58b0c9c1fa6cc13759ead9b42db9eebe] (r:0 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca666fdcbb473985b3ac933d13f4acff8d] (r:0 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca98c2640cda6c0d801194a8a61c699224] (r:0 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcacddc49c5f30807d474a09d70fed8a569] (r:0 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcad642c00af119adf30dc11d32e9f0886d] (r:0 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaed441ceb81326c56263efbb60c95c2e4] (r:0 w:1)
	fn set_staking_configs_all_remove() -> Weight {
		(4_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca422adb579f1dbf4f3886c5cfa3bb8cc4] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca9c6a637f62ae2af1c7e31eed7e96be04] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcacddc49c5f30807d474a09d70fed8a569] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcad642c00af119adf30dc11d32e9f0886d] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaf99b25852d3d69419882da651375cdb3] (r:1 w:1)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedcaed441ceb81326c56263efbb60c95c2e4] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca88dcde934c658227ee1dfafcd6e16903] (r:1 w:0)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685e62556a85fcb7c61b2c6c750924846b15] (r:2 w:2)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685ee5c03730c8f59f00941607850b6633d8] (r:1 w:1)
	// Storage: unknown [0xdf66cf37cde77d2a63889732a23c685e7a6dc62e324093ba1331bf49fdb2f24a] (r:1 w:1)
	fn chill_other() -> Weight {
		(55_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca58b0c9c1fa6cc13759ead9b42db9eebe] (r:1 w:0)
	// Storage: unknown [0x5f3e4907f716ac89b6347d15ececedca88dcde934c658227ee1dfafcd6e16903] (r:1 w:1)
	fn force_apply_min_commission() -> Weight {
		(7_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
