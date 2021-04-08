// Copyright (C) 2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A list of the different weight modules for our runtime.

pub mod fabric_system;
pub mod noble_balances;
pub mod noble_bounties;
pub mod noble_collective;
pub mod noble_democracy;
pub mod noble_elections_phragmen;
pub mod noble_identity;
pub mod noble_im_online;
pub mod noble_indices;
pub mod noble_multisig;
pub mod noble_proxy;
pub mod noble_scheduler;
pub mod noble_session;
pub mod noble_staking;
pub mod noble_timestamp;
pub mod noble_tips;
pub mod noble_treasury;
pub mod noble_utility;
pub mod noble_vesting;
pub mod runtime_common_claims;
