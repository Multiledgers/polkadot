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
//! Autogenerated weights for `pallet_vesting`
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
// --pallet=pallet_vesting
// --extrinsic=*
// --execution=native
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_vesting`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_vesting::WeightInfo for WeightInfo<T> {
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_locked(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `377 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `5327 + l * (25 ±0) + s * (36 ±0)`
		// Minimum execution time: 22_031 nanoseconds.
		Weight::from_parts(20_222_790, 5327)
			// Standard Error: 4_542
			.saturating_add(Weight::from_ref_time(44_353).saturating_mul(l.into()))
			// Standard Error: 8_330
			.saturating_add(Weight::from_ref_time(66_859).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_proof_size(25).saturating_mul(l.into()))
			.saturating_add(Weight::from_proof_size(36).saturating_mul(s.into()))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_unlocked(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `377 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `5327 + l * (25 ±0) + s * (36 ±0)`
		// Minimum execution time: 21_250 nanoseconds.
		Weight::from_parts(19_899_747, 5327)
			// Standard Error: 4_073
			.saturating_add(Weight::from_ref_time(44_265).saturating_mul(l.into()))
			// Standard Error: 7_470
			.saturating_add(Weight::from_ref_time(49_060).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_proof_size(25).saturating_mul(l.into()))
			.saturating_add(Weight::from_proof_size(36).saturating_mul(s.into()))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_other_locked(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `512 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `7937 + l * (25 ±0) + s * (36 ±0)`
		// Minimum execution time: 24_496 nanoseconds.
		Weight::from_parts(22_555_096, 7937)
			// Standard Error: 3_974
			.saturating_add(Weight::from_ref_time(49_931).saturating_mul(l.into()))
			// Standard Error: 7_287
			.saturating_add(Weight::from_ref_time(64_197).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(25).saturating_mul(l.into()))
			.saturating_add(Weight::from_proof_size(36).saturating_mul(s.into()))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_other_unlocked(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `512 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `7937 + l * (25 ±0) + s * (36 ±0)`
		// Minimum execution time: 23_945 nanoseconds.
		Weight::from_parts(23_382_027, 7937)
			// Standard Error: 39_063
			.saturating_add(Weight::from_ref_time(15_490).saturating_mul(l.into()))
			// Standard Error: 71_631
			.saturating_add(Weight::from_ref_time(70_301).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(25).saturating_mul(l.into()))
			.saturating_add(Weight::from_proof_size(36).saturating_mul(s.into()))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[0, 27]`.
	fn vested_transfer(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `583 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `8008 + l * (25 ±0) + s * (36 ±0)`
		// Minimum execution time: 31_680 nanoseconds.
		Weight::from_parts(31_657_465, 8008)
			// Standard Error: 7_658
			.saturating_add(Weight::from_ref_time(44_730).saturating_mul(l.into()))
			// Standard Error: 14_044
			.saturating_add(Weight::from_ref_time(23_933).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(25).saturating_mul(l.into()))
			.saturating_add(Weight::from_proof_size(36).saturating_mul(s.into()))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[0, 27]`.
	fn force_vested_transfer(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `718 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `10618 + l * (25 ±0) + s * (36 ±0)`
		// Minimum execution time: 34_635 nanoseconds.
		Weight::from_parts(33_856_839, 10618)
			// Standard Error: 17_575
			.saturating_add(Weight::from_ref_time(54_646).saturating_mul(l.into()))
			// Standard Error: 32_228
			.saturating_add(Weight::from_ref_time(30_261).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_proof_size(25).saturating_mul(l.into()))
			.saturating_add(Weight::from_proof_size(36).saturating_mul(s.into()))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[2, 28]`.
	fn not_unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `510 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `7935 + l * (25 ±0) + s * (36 ±0)`
		// Minimum execution time: 24_487 nanoseconds.
		Weight::from_parts(22_640_144, 7935)
			// Standard Error: 10_216
			.saturating_add(Weight::from_ref_time(54_129).saturating_mul(l.into()))
			// Standard Error: 18_961
			.saturating_add(Weight::from_ref_time(55_577).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(25).saturating_mul(l.into()))
			.saturating_add(Weight::from_proof_size(36).saturating_mul(s.into()))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[2, 28]`.
	fn unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `510 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `7935 + l * (25 ±0) + s * (36 ±0)`
		// Minimum execution time: 24_326 nanoseconds.
		Weight::from_parts(22_387_176, 7935)
			// Standard Error: 10_082
			.saturating_add(Weight::from_ref_time(61_996).saturating_mul(l.into()))
			// Standard Error: 18_714
			.saturating_add(Weight::from_ref_time(65_680).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(25).saturating_mul(l.into()))
			.saturating_add(Weight::from_proof_size(36).saturating_mul(s.into()))
	}
}
