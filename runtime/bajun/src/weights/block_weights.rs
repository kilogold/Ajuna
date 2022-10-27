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

pub mod constants {
	use frame_support::{
		parameter_types,
		weights::{constants, Weight},
	};

	parameter_types! {
		/// Importing a block with 0 Extrinsics.
		pub const BlockExecutionWeight: Weight = 5_000_000 * constants::WEIGHT_PER_NANOS;
	}

	#[cfg(test)]
	mod test_weights {
		use frame_support::weights::constants;

		/// Checks that the weight exists and is sane.
		// NOTE: If this test fails but you are sure that the generated values are fine,
		// you can delete it.
		#[test]
		fn sane() {
			let w = super::constants::BlockExecutionWeight::get();

			// At least 100 µs.
			assert!(w >= 100 * constants::WEIGHT_PER_MICROS, "Weight should be at least 100 µs.");
			// At most 50 ms.
			assert!(w <= 50 * constants::WEIGHT_PER_MILLIS, "Weight should be at most 50 ms.");
		}
	}
}
