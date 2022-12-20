// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_proxy`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-20, STEPS: `10`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `6267FC`, CPU: `AMD Ryzen 5 PRO 3600 6-Core Processor`
//! EXECUTION: Some(Native), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/polkadot
// benchmark
// pallet
// --chain=polkadot-dev
// --steps=10
// --repeat=1
// --pallet=pallet_proxy
// --extrinsic=*
// --execution=native
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_proxy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_proxy::WeightInfo for WeightInfo<T> {
	/// Storage: Proxy Proxies (r:1 w:0)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716)
	/// The range of component `p` is `[1, 31]`.
	fn proxy(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `259 + p * (37 ±0)`
		//  Estimated: `2734 + p * (37 ±0)`
		// Minimum execution time: 11_892 nanoseconds.
		Weight::from_parts(12_238_696, 2734)
			// Standard Error: 6_884
			.saturating_add(Weight::from_ref_time(26_255).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(Weight::from_proof_size(37).saturating_mul(p.into()))
	}
	/// Storage: Proxy Proxies (r:1 w:0)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716)
	/// Storage: Proxy Announcements (r:1 w:1)
	/// Proof: Proxy Announcements (max_values: None, max_size: Some(2233), added: 4708)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn proxy_announced(a: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `650 + a * (68 ±0) + p * (37 ±0)`
		//  Estimated: `8075 + a * (68 ±0) + p * (37 ±0)`
		// Minimum execution time: 24_757 nanoseconds.
		Weight::from_parts(23_421_937, 8075)
			// Standard Error: 16_134
			.saturating_add(Weight::from_ref_time(86_694).saturating_mul(a.into()))
			// Standard Error: 16_773
			.saturating_add(Weight::from_ref_time(40_886).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_proof_size(68).saturating_mul(a.into()))
			.saturating_add(Weight::from_proof_size(37).saturating_mul(p.into()))
	}
	/// Storage: Proxy Announcements (r:1 w:1)
	/// Proof: Proxy Announcements (max_values: None, max_size: Some(2233), added: 4708)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn remove_announcement(a: u32, _p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `533 + a * (68 ±0)`
		//  Estimated: `5483 + a * (68 ±0)`
		// Minimum execution time: 15_690 nanoseconds.
		Weight::from_parts(18_920_099, 5483)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_proof_size(68).saturating_mul(a.into()))
	}
	/// Storage: Proxy Announcements (r:1 w:1)
	/// Proof: Proxy Announcements (max_values: None, max_size: Some(2233), added: 4708)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn reject_announcement(a: u32, _p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `533 + a * (68 ±0)`
		//  Estimated: `5483 + a * (68 ±0)`
		// Minimum execution time: 16_241 nanoseconds.
		Weight::from_parts(21_651_579, 5483)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_proof_size(68).saturating_mul(a.into()))
	}
	/// Storage: Proxy Proxies (r:1 w:0)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716)
	/// Storage: Proxy Announcements (r:1 w:1)
	/// Proof: Proxy Announcements (max_values: None, max_size: Some(2233), added: 4708)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn announce(a: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `582 + a * (68 ±0) + p * (37 ±0)`
		//  Estimated: `8007 + a * (68 ±0) + p * (37 ±0)`
		// Minimum execution time: 21_370 nanoseconds.
		Weight::from_parts(20_832_368, 8007)
			// Standard Error: 10_112
			.saturating_add(Weight::from_ref_time(75_388).saturating_mul(a.into()))
			// Standard Error: 10_513
			.saturating_add(Weight::from_ref_time(37_921).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_proof_size(68).saturating_mul(a.into()))
			.saturating_add(Weight::from_proof_size(37).saturating_mul(p.into()))
	}
	/// Storage: Proxy Proxies (r:1 w:1)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716)
	/// The range of component `p` is `[1, 31]`.
	fn add_proxy(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `259 + p * (37 ±0)`
		//  Estimated: `2734 + p * (37 ±0)`
		// Minimum execution time: 16_171 nanoseconds.
		Weight::from_parts(17_407_916, 2734)
			// Standard Error: 160_564
			.saturating_add(Weight::from_ref_time(209_323).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_proof_size(37).saturating_mul(p.into()))
	}
	/// Storage: Proxy Proxies (r:1 w:1)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716)
	/// The range of component `p` is `[1, 31]`.
	fn remove_proxy(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `259 + p * (37 ±0)`
		//  Estimated: `2734 + p * (37 ±0)`
		// Minimum execution time: 16_671 nanoseconds.
		Weight::from_parts(17_290_666, 2734)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_proof_size(37).saturating_mul(p.into()))
	}
	/// Storage: Proxy Proxies (r:1 w:1)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716)
	/// The range of component `p` is `[1, 31]`.
	fn remove_proxies(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `259 + p * (37 ±0)`
		//  Estimated: `2734 + p * (37 ±0)`
		// Minimum execution time: 13_535 nanoseconds.
		Weight::from_parts(14_145_904, 2734)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_proof_size(37).saturating_mul(p.into()))
	}
	/// Storage: Proxy Proxies (r:1 w:1)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716)
	/// The range of component `p` is `[1, 31]`.
	fn create_pure(_p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `239`
		//  Estimated: `2714`
		// Minimum execution time: 16_401 nanoseconds.
		Weight::from_parts(17_729_511, 2714)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Proxy Proxies (r:1 w:1)
	/// Proof: Proxy Proxies (max_values: None, max_size: Some(1241), added: 3716)
	/// The range of component `p` is `[0, 30]`.
	fn kill_pure(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `296 + p * (37 ±0)`
		//  Estimated: `2771 + p * (37 ±0)`
		// Minimum execution time: 13_716 nanoseconds.
		Weight::from_parts(22_001_315, 2771)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_proof_size(37).saturating_mul(p.into()))
	}
}
