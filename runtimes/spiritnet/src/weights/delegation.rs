// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2022 BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

//! Autogenerated weights for delegation
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-23, STEPS: `1`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("spiritnet-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// pallet
// --chain=spiritnet-dev
// --steps=1
// --repeat=20
// --pallet=delegation
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtimes/spiritnet/src/weights/delegation.rs
// --template=.maintain/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `delegation`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> delegation::WeightInfo for WeightInfo<T> {
	// Storage: Delegation DelegationHierarchies (r:1 w:1)
	// Storage: Ctype Ctypes (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Delegation DelegationNodes (r:0 w:1)
	fn create_hierarchy() -> Weight {
		(41_504_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Delegation DelegationNodes (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn add_delegation() -> Weight {
		(48_679_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Delegation DelegationNodes (r:1 w:1)
	// Storage: Delegation DelegationHierarchies (r:1 w:0)
	fn revoke_delegation_root_child(r: u32, _c: u32, ) -> Weight {
		(20_626_000 as Weight)
			// Standard Error: 30_000
			.saturating_add((14_142_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(r as Weight)))
	}
	// Storage: Delegation DelegationNodes (r:6 w:1)
	// Storage: Delegation DelegationHierarchies (r:1 w:0)
	fn revoke_delegation_leaf(_r: u32, c: u32, ) -> Weight {
		(34_444_000 as Weight)
			// Standard Error: 24_000
			.saturating_add((4_926_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Delegation DelegationNodes (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: Delegation DelegationHierarchies (r:1 w:1)
	fn remove_delegation(r: u32, ) -> Weight {
		(55_382_000 as Weight)
			// Standard Error: 46_000
			.saturating_add((24_306_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(r as Weight)))
	}
	// Storage: Delegation DelegationNodes (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: Delegation DelegationHierarchies (r:0 w:1)
	fn reclaim_deposit(r: u32, ) -> Weight {
		(47_460_000 as Weight)
			// Standard Error: 56_000
			.saturating_add((24_567_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(r as Weight)))
	}
	// Storage: Delegation DelegationNodes (r:1 w:0)
	// Storage: Delegation DelegationHierarchies (r:1 w:0)
	fn can_attest() -> Weight {
		(12_951_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: Delegation DelegationNodes (r:2 w:0)
	fn can_revoke(c: u32, ) -> Weight {
		(8_132_000 as Weight)
			// Standard Error: 22_000
			.saturating_add((4_897_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(c as Weight)))
	}
	// Storage: Delegation DelegationNodes (r:2 w:0)
	fn can_remove(c: u32, ) -> Weight {
		(8_055_000 as Weight)
			// Standard Error: 19_000
			.saturating_add((4_947_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(c as Weight)))
	}
}
