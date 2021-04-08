// Copyright 2020 Parity Technologies (UK) Ltd.
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

//! The `Error` and `Result` types used by the subsystem.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error(transparent)]
	Subsystem(#[from] tetcoin_subsystem::SubsystemError),
	#[error(transparent)]
	OneshotRecv(#[from] futures::channel::oneshot::Canceled),
	#[error(transparent)]
	Runtime(#[from] tetcoin_subsystem::errors::RuntimeApiError),
	#[error(transparent)]
	Util(#[from] tetcoin_node_subsystem_util::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
