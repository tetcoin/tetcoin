// Copyright 2017-2020 Parity Technologies (UK) Ltd.
// This file is part of Tetcoin.

// Tetcoin is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tetcoin is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tetcoin.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for noble_proxy
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.0
//! DATE: 2020-12-09, STEPS: [50, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("tetcoin-dev"), DB CACHE: 128

// Executed Command:
// target/release/tetcoin
// benchmark
// --chain=tetcoin-dev
// --steps=50
// --repeat=20
// --pallet=noble_proxy
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/tetcoin/src/weights/


#![allow(unused_parens)]
#![allow(unused_imports)]

use fabric_support::{traits::Get, weights::Weight};
use tetcore_std::marker::PhantomData;

/// Weight functions for noble_proxy.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: fabric_system::Config> noble_proxy::WeightInfo for WeightInfo<T> {
	fn proxy(p: u32, ) -> Weight {
		(31_560_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((190_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	fn proxy_announced(a: u32, p: u32, ) -> Weight {
		(65_555_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((843_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 1_000
			.saturating_add((194_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn remove_announcement(a: u32, p: u32, ) -> Weight {
		(41_808_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((842_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 1_000
			.saturating_add((10_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn reject_announcement(a: u32, p: u32, ) -> Weight {
		(41_713_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((847_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 1_000
			.saturating_add((12_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn announce(a: u32, p: u32, ) -> Weight {
		(66_579_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((730_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 1_000
			.saturating_add((199_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn add_proxy(p: u32, ) -> Weight {
		(44_930_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((206_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn remove_proxy(p: u32, ) -> Weight {
		(40_436_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((241_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn remove_proxies(p: u32, ) -> Weight {
		(38_695_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((191_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn anonymous(p: u32, ) -> Weight {
		(64_695_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((13_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn kill_anonymous(p: u32, ) -> Weight {
		(41_503_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((192_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}