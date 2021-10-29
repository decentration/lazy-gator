// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2021 BOTLabs GmbH

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

//! Autogenerated weights for did
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-10-27, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("spiritnet-dev"), DB CACHE: 128

// Executed Command:
// target/release/kilt-parachain
// benchmark
// --chain=spiritnet-dev
// --steps=50
// --repeat=20
// --pallet=did
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtimes/spiritnet/src/weights/did.rs
// --template=.maintain/runtime-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for did using the recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> did::WeightInfo for WeightInfo<T> {
	fn create_ed25519_keys(n: u32, c: u32, ) -> Weight {
		(155_200_000_u64)
			// Standard Error: 29_000
			.saturating_add((2_379_000_u64).saturating_mul(n as Weight))
			// Standard Error: 9_000
			.saturating_add((10_503_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c as Weight)))
	}
	fn create_sr25519_keys(n: u32, c: u32, ) -> Weight {
		(158_638_000_u64)
			// Standard Error: 31_000
			.saturating_add((2_385_000_u64).saturating_mul(n as Weight))
			// Standard Error: 9_000
			.saturating_add((10_847_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c as Weight)))
	}
	fn create_ecdsa_keys(n: u32, c: u32, ) -> Weight {
		(273_495_000_u64)
			// Standard Error: 35_000
			.saturating_add((2_403_000_u64).saturating_mul(n as Weight))
			// Standard Error: 11_000
			.saturating_add((10_325_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c as Weight)))
	}
	fn delete(c: u32, ) -> Weight {
		(41_042_000_u64)
			// Standard Error: 4_000
			.saturating_add((1_095_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c as Weight)))
	}
	fn reclaim_deposit(c: u32, ) -> Weight {
		(45_858_000_u64)
			// Standard Error: 5_000
			.saturating_add((1_099_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c as Weight)))
	}
	fn submit_did_call_ed25519_key() -> Weight {
		(86_443_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn submit_did_call_sr25519_key() -> Weight {
		(89_533_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn submit_did_call_ecdsa_key() -> Weight {
		(204_492_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_ed25519_authentication_key() -> Weight {
		(48_157_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_sr25519_authentication_key() -> Weight {
		(48_387_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_ecdsa_authentication_key() -> Weight {
		(48_544_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_ed25519_delegation_key() -> Weight {
		(47_631_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_sr25519_delegation_key() -> Weight {
		(48_186_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_ecdsa_delegation_key() -> Weight {
		(48_042_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_ed25519_delegation_key() -> Weight {
		(44_720_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_sr25519_delegation_key() -> Weight {
		(44_742_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_ecdsa_delegation_key() -> Weight {
		(44_476_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_ed25519_attestation_key() -> Weight {
		(48_241_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_sr25519_attestation_key() -> Weight {
		(48_137_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_ecdsa_attestation_key() -> Weight {
		(48_281_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_ed25519_attestation_key() -> Weight {
		(44_762_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_sr25519_attestation_key() -> Weight {
		(44_854_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_ecdsa_attestation_key() -> Weight {
		(44_720_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn add_ed25519_key_agreement_key() -> Weight {
		(47_004_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn add_sr25519_key_agreement_key() -> Weight {
		(47_082_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn add_ecdsa_key_agreement_key() -> Weight {
		(47_217_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_ed25519_key_agreement_key() -> Weight {
		(45_138_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_sr25519_key_agreement_key() -> Weight {
		(44_988_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_ecdsa_key_agreement_key() -> Weight {
		(44_926_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn add_service_endpoint() -> Weight {
		(43_394_000_u64)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn remove_service_endpoint() -> Weight {
		(34_936_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn signature_verification_sr25519(l: u32, ) -> Weight {
		(25_599_000_u64)
			// Standard Error: 0
			.saturating_add((4_000_u64).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	fn signature_verification_ed25519(l: u32, ) -> Weight {
		(21_307_000_u64)
			// Standard Error: 0
			.saturating_add((2_000_u64).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	fn signature_verification_ecdsa(l: u32, ) -> Weight {
		(138_685_000_u64)
			// Standard Error: 0
			.saturating_add((1_000_u64).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
}