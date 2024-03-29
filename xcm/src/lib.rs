// Copyright 2020 Parity Technologies (UK) Ltd.
// This file is part of Tetcoin.

// Tetcore is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tetcore is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tetcoin.  If not, see <http://www.gnu.org/licenses/>.

//! Cross-Consensus Message format data structures.

// NOTE, this crate is meant to be used in many different environments, notably wasm, but not
// necessarily related to FRAME or even Tetcore.
//
// Hence, `no_std` rather than tp-runtime.
#![no_std]
extern crate alloc;

use tetsy_scale_codec::{Encode, Decode};

pub mod v0;

/// A single XCM message, together with its version code.
#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug)]
pub enum VersionedXcm {
	V0(v0::Xcm),
}

/// A versioned multi-location, a relative location of a cross-consensus system identifier.
#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug)]
pub enum VersionedMultiLocation {
	V0(v0::MultiLocation),
}

/// A versioned multi-asset, an identifier for an asset within a consensus system.
#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug)]
pub enum VersionedMultiAsset {
	V0(v0::MultiAsset),
}
