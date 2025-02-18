// Ajuna Node
// Copyright (C) 2022 BlogaTech AG

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_nfts`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-10, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `weight-calculation`, CPU: `DO-Regular`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/bajun-para
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet-nfts
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./HEADER-AGPL
// --output=./runtime/bajun/src/weights/pallet_nfts.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_nfts`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_nfts::WeightInfo for WeightInfo<T> {
	// Storage: NFT NextCollectionId (r:1 w:1)
	// Storage: NFT Collection (r:1 w:1)
	// Storage: NFT CollectionRoleOf (r:0 w:1)
	// Storage: NFT CollectionConfigOf (r:0 w:1)
	// Storage: NFT CollectionAccount (r:0 w:1)
	fn create() -> Weight {
		// Minimum execution time: 71_267 nanoseconds.
		Weight::from_ref_time(74_871_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: NFT NextCollectionId (r:1 w:1)
	// Storage: NFT Collection (r:1 w:1)
	// Storage: NFT CollectionRoleOf (r:0 w:1)
	// Storage: NFT CollectionConfigOf (r:0 w:1)
	// Storage: NFT CollectionAccount (r:0 w:1)
	fn force_create() -> Weight {
		// Minimum execution time: 71_060 nanoseconds.
		Weight::from_ref_time(86_591_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: NFT Collection (r:1 w:1)
	// Storage: NFT Item (r:1001 w:1000)
	// Storage: NFT Attribute (r:1001 w:1000)
	// Storage: NFT ItemMetadataOf (r:0 w:1000)
	// Storage: NFT CollectionRoleOf (r:0 w:1)
	// Storage: NFT CollectionMetadataOf (r:0 w:1)
	// Storage: NFT CollectionConfigOf (r:0 w:1)
	// Storage: NFT ItemConfigOf (r:0 w:1000)
	// Storage: NFT Account (r:0 w:1000)
	// Storage: NFT CollectionAccount (r:0 w:1)
	/// The range of component `n` is `[0, 1000]`.
	/// The range of component `m` is `[0, 1000]`.
	/// The range of component `a` is `[0, 1000]`.
	fn destroy(_n: u32, m: u32, a: u32, ) -> Weight {
		// Minimum execution time: 37_870_444 nanoseconds.
		Weight::from_ref_time(38_593_122_480)
			// Standard Error: 285_747
			.saturating_add(Weight::from_ref_time(1_092_652).saturating_mul(m.into()))
			// Standard Error: 285_747
			.saturating_add(Weight::from_ref_time(17_427_683).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(1003))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes(3005))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(a.into())))
	}
	// Storage: NFT CollectionConfigOf (r:1 w:0)
	// Storage: NFT Item (r:1 w:1)
	// Storage: NFT Collection (r:1 w:1)
	// Storage: NFT CollectionRoleOf (r:1 w:0)
	// Storage: NFT ItemConfigOf (r:1 w:1)
	// Storage: NFT Account (r:0 w:1)
	fn mint() -> Weight {
		// Minimum execution time: 92_638 nanoseconds.
		Weight::from_ref_time(96_080_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: NFT CollectionRoleOf (r:1 w:0)
	// Storage: NFT Item (r:1 w:1)
	// Storage: NFT Collection (r:1 w:1)
	// Storage: NFT CollectionConfigOf (r:1 w:0)
	// Storage: NFT ItemConfigOf (r:1 w:1)
	// Storage: NFT Account (r:0 w:1)
	fn force_mint() -> Weight {
		// Minimum execution time: 83_662 nanoseconds.
		Weight::from_ref_time(94_731_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: NFT Collection (r:1 w:1)
	// Storage: NFT Item (r:1 w:1)
	// Storage: NFT CollectionRoleOf (r:1 w:0)
	// Storage: NFT ItemConfigOf (r:1 w:1)
	// Storage: NFT Account (r:0 w:1)
	// Storage: NFT ItemPriceOf (r:0 w:1)
	// Storage: NFT ItemAttributesApprovalsOf (r:0 w:1)
	// Storage: NFT PendingSwapOf (r:0 w:1)
	fn burn() -> Weight {
		// Minimum execution time: 88_907 nanoseconds.
		Weight::from_ref_time(100_461_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: NFT Collection (r:1 w:0)
	// Storage: NFT CollectionConfigOf (r:1 w:0)
	// Storage: NFT ItemConfigOf (r:1 w:0)
	// Storage: NFT Item (r:1 w:1)
	// Storage: NFT CollectionRoleOf (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: NFT Account (r:0 w:2)
	// Storage: NFT ItemPriceOf (r:0 w:1)
	// Storage: NFT PendingSwapOf (r:0 w:1)
	fn transfer() -> Weight {
		// Minimum execution time: 97_690 nanoseconds.
		Weight::from_ref_time(110_353_000)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: NFT Collection (r:1 w:0)
	// Storage: NFT CollectionConfigOf (r:1 w:0)
	// Storage: NFT Item (r:102 w:102)
	/// The range of component `i` is `[0, 5000]`.
	fn redeposit(i: u32, ) -> Weight {
		// Minimum execution time: 34_582 nanoseconds.
		Weight::from_ref_time(34_852_000)
			// Standard Error: 37_628
			.saturating_add(Weight::from_ref_time(25_381_432).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
	}
	// Storage: NFT CollectionRoleOf (r:1 w:0)
	// Storage: NFT ItemConfigOf (r:1 w:1)
	fn lock_item_transfer() -> Weight {
		// Minimum execution time: 41_497 nanoseconds.
		Weight::from_ref_time(45_827_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: NFT CollectionRoleOf (r:1 w:0)
	// Storage: NFT ItemConfigOf (r:1 w:1)
	fn unlock_item_transfer() -> Weight {
		// Minimum execution time: 41_665 nanoseconds.
		Weight::from_ref_time(45_998_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: NFT CollectionRoleOf (r:1 w:0)
	// Storage: NFT CollectionConfigOf (r:1 w:1)
	fn lock_collection() -> Weight {
		// Minimum execution time: 37_321 nanoseconds.
		Weight::from_ref_time(41_489_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: NFT OwnershipAcceptance (r:1 w:1)
	// Storage: NFT Collection (r:1 w:1)
	// Storage: NFT CollectionAccount (r:0 w:2)
	fn transfer_ownership() -> Weight {
		// Minimum execution time: 45_522 nanoseconds.
		Weight::from_ref_time(50_428_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: NFT Collection (r:1 w:1)
	// Storage: NFT CollectionRoleOf (r:0 w:4)
	fn set_team() -> Weight {
		// Minimum execution time: 51_008 nanoseconds.
		Weight::from_ref_time(58_741_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: NFT Collection (r:1 w:1)
	// Storage: NFT CollectionAccount (r:0 w:2)
	fn force_collection_owner() -> Weight {
		// Minimum execution time: 38_601 nanoseconds.
		Weight::from_ref_time(42_451_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: NFT Collection (r:1 w:0)
	// Storage: NFT CollectionConfigOf (r:0 w:1)
	fn force_collection_config() -> Weight {
		// Minimum execution time: 32_955 nanoseconds.
		Weight::from_ref_time(37_666_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: NFT Collection (r:1 w:0)
	// Storage: NFT ItemConfigOf (r:1 w:1)
	fn lock_item_properties() -> Weight {
		// Minimum execution time: 39_665 nanoseconds.
		Weight::from_ref_time(44_965_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: NFT Collection (r:1 w:1)
	// Storage: NFT CollectionConfigOf (r:1 w:0)
	// Storage: NFT ItemConfigOf (r:1 w:0)
	// Storage: NFT Attribute (r:1 w:1)
	fn set_attribute() -> Weight {
		// Minimum execution time: 83_116 nanoseconds.
		Weight::from_ref_time(94_269_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: NFT Collection (r:1 w:1)
	// Storage: NFT Attribute (r:1 w:1)
	fn force_set_attribute() -> Weight {
		// Minimum execution time: 54_921 nanoseconds.
		Weight::from_ref_time(60_867_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: NFT Attribute (r:1 w:1)
	// Storage: NFT Collection (r:1 w:1)
	// Storage: NFT ItemConfigOf (r:1 w:0)
	fn clear_attribute() -> Weight {
		// Minimum execution time: 75_584 nanoseconds.
		Weight::from_ref_time(85_221_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: NFT Item (r:1 w:0)
	// Storage: NFT ItemAttributesApprovalsOf (r:1 w:1)
	fn approve_item_attributes() -> Weight {
		// Minimum execution time: 38_978 nanoseconds.
		Weight::from_ref_time(42_738_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: NFT Item (r:1 w:0)
	// Storage: NFT ItemAttributesApprovalsOf (r:1 w:1)
	// Storage: NFT Attribute (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	/// The range of component `n` is `[0, 1000]`.
	fn cancel_item_attributes_approval(n: u32, ) -> Weight {
		// Minimum execution time: 53_689 nanoseconds.
		Weight::from_ref_time(54_573_000)
			// Standard Error: 34_366
			.saturating_add(Weight::from_ref_time(13_350_882).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
	}
	// Storage: NFT Collection (r:1 w:1)
	// Storage: NFT ItemConfigOf (r:1 w:0)
	// Storage: NFT CollectionConfigOf (r:1 w:0)
	// Storage: NFT ItemMetadataOf (r:1 w:1)
	fn set_metadata() -> Weight {
		// Minimum execution time: 69_800 nanoseconds.
		Weight::from_ref_time(77_001_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: NFT Collection (r:1 w:1)
	// Storage: NFT ItemConfigOf (r:1 w:0)
	// Storage: NFT ItemMetadataOf (r:1 w:1)
	fn clear_metadata() -> Weight {
		// Minimum execution time: 66_168 nanoseconds.
		Weight::from_ref_time(73_811_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: NFT CollectionConfigOf (r:1 w:0)
	// Storage: NFT Collection (r:1 w:1)
	// Storage: NFT CollectionMetadataOf (r:1 w:1)
	fn set_collection_metadata() -> Weight {
		// Minimum execution time: 63_059 nanoseconds.
		Weight::from_ref_time(65_972_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: NFT Collection (r:1 w:0)
	// Storage: NFT CollectionConfigOf (r:1 w:0)
	// Storage: NFT CollectionMetadataOf (r:1 w:1)
	fn clear_collection_metadata() -> Weight {
		// Minimum execution time: 60_519 nanoseconds.
		Weight::from_ref_time(68_763_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: NFT Item (r:1 w:1)
	// Storage: NFT CollectionConfigOf (r:1 w:0)
	// Storage: NFT CollectionRoleOf (r:1 w:0)
	fn approve_transfer() -> Weight {
		// Minimum execution time: 49_377 nanoseconds.
		Weight::from_ref_time(55_263_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: NFT Item (r:1 w:1)
	// Storage: NFT CollectionRoleOf (r:1 w:0)
	fn cancel_approval() -> Weight {
		// Minimum execution time: 50_421 nanoseconds.
		Weight::from_ref_time(52_182_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: NFT Item (r:1 w:1)
	// Storage: NFT CollectionRoleOf (r:1 w:0)
	fn clear_all_transfer_approvals() -> Weight {
		// Minimum execution time: 47_883 nanoseconds.
		Weight::from_ref_time(49_307_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: NFT OwnershipAcceptance (r:1 w:1)
	fn set_accept_ownership() -> Weight {
		// Minimum execution time: 37_603 nanoseconds.
		Weight::from_ref_time(41_518_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: NFT CollectionConfigOf (r:1 w:1)
	// Storage: NFT Collection (r:1 w:0)
	fn set_collection_max_supply() -> Weight {
		// Minimum execution time: 39_066 nanoseconds.
		Weight::from_ref_time(43_579_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: NFT Collection (r:1 w:0)
	// Storage: NFT CollectionConfigOf (r:1 w:1)
	fn update_mint_settings() -> Weight {
		// Minimum execution time: 40_624 nanoseconds.
		Weight::from_ref_time(42_548_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: NFT Item (r:1 w:0)
	// Storage: NFT CollectionConfigOf (r:1 w:0)
	// Storage: NFT ItemConfigOf (r:1 w:0)
	// Storage: NFT ItemPriceOf (r:0 w:1)
	fn set_price() -> Weight {
		// Minimum execution time: 47_442 nanoseconds.
		Weight::from_ref_time(53_840_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: NFT Item (r:1 w:1)
	// Storage: NFT ItemPriceOf (r:1 w:1)
	// Storage: NFT Collection (r:1 w:0)
	// Storage: NFT CollectionConfigOf (r:1 w:0)
	// Storage: NFT ItemConfigOf (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: NFT Account (r:0 w:2)
	// Storage: NFT PendingSwapOf (r:0 w:1)
	fn buy_item() -> Weight {
		// Minimum execution time: 116_941 nanoseconds.
		Weight::from_ref_time(122_533_000)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// The range of component `n` is `[0, 1]`.
	fn pay_tips(n: u32, ) -> Weight {
		// Minimum execution time: 7_019 nanoseconds.
		Weight::from_ref_time(7_904_565)
			// Standard Error: 87_209
			.saturating_add(Weight::from_ref_time(18_621_534).saturating_mul(n.into()))
	}
	// Storage: NFT Item (r:2 w:0)
	// Storage: NFT PendingSwapOf (r:0 w:1)
	fn create_swap() -> Weight {
		// Minimum execution time: 48_710 nanoseconds.
		Weight::from_ref_time(51_451_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: NFT PendingSwapOf (r:1 w:1)
	// Storage: NFT Item (r:1 w:0)
	fn cancel_swap() -> Weight {
		// Minimum execution time: 51_245 nanoseconds.
		Weight::from_ref_time(54_222_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: NFT Item (r:2 w:2)
	// Storage: NFT PendingSwapOf (r:1 w:2)
	// Storage: NFT Collection (r:1 w:0)
	// Storage: NFT CollectionConfigOf (r:1 w:0)
	// Storage: NFT ItemConfigOf (r:2 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: NFT Account (r:0 w:4)
	// Storage: NFT ItemPriceOf (r:0 w:2)
	fn claim_swap() -> Weight {
		// Minimum execution time: 169_053 nanoseconds.
		Weight::from_ref_time(175_041_000)
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(11))
	}
}
