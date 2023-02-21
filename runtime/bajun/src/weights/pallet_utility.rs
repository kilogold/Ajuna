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

//! Autogenerated weights for `pallet_utility`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `weight-calculation`, CPU: `DO-Regular`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/bajun-para
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet-utility
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./HEADER-AGPL
// --output=./runtime/bajun/src/weights/pallet_utility.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_utility`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for WeightInfo<T> {
	/// The range of component `c` is `[0, 1000]`.
	fn batch(c: u32, ) -> Weight {
		// Minimum execution time: 25_014 nanoseconds.
		Weight::from_ref_time(294_552_863)
			// Standard Error: 50_283
			.saturating_add(Weight::from_ref_time(8_489_493).saturating_mul(c.into()))
	}
	fn as_derivative() -> Weight {
		// Minimum execution time: 12_921 nanoseconds.
		Weight::from_ref_time(13_351_000)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn batch_all(c: u32, ) -> Weight {
		// Minimum execution time: 25_004 nanoseconds.
		Weight::from_ref_time(25_286_000)
			// Standard Error: 76_591
			.saturating_add(Weight::from_ref_time(9_922_749).saturating_mul(c.into()))
	}
	fn dispatch_as() -> Weight {
		// Minimum execution time: 29_222 nanoseconds.
		Weight::from_ref_time(30_120_000)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		// Minimum execution time: 24_935 nanoseconds.
		Weight::from_ref_time(151_756_515)
			// Standard Error: 61_414
			.saturating_add(Weight::from_ref_time(8_627_236).saturating_mul(c.into()))
	}
}
