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

//! Tetcoin Client meta trait

use std::sync::Arc;
use tp_api::{ProvideRuntimeApi, CallApiAt, NumberFor};
use tp_blockchain::HeaderBackend;
use tp_runtime::{
	Justification, generic::{BlockId, SignedBlock}, traits::{Block as BlockT, BlakeTwo256},
};
use tc_client_api::{Backend as BackendT, BlockchainEvents, KeyIterator};
use tp_storage::{StorageData, StorageKey, ChildInfo, PrefixedStorageKey};
use tetcoin_primitives::v1::{Block, ParachainHost, AccountId, Nonce, Balance, Header, BlockNumber, Hash};
use consensus_common::BlockStatus;

/// A set of APIs that tetcoin-like runtimes must implement.
pub trait RuntimeApiCollection:
	tp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
	+ tp_api::ApiExt<Block, Error = tp_blockchain::Error>
	+ babe_primitives::BabeApi<Block>
	+ grandpa_primitives::GrandpaApi<Block>
	+ ParachainHost<Block>
	+ tp_block_builder::BlockBuilder<Block>
	+ fabric_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce>
	+ noble_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
	+ tp_api::Metadata<Block>
	+ tp_offchain::OffchainWorkerApi<Block>
	+ tp_session::SessionKeys<Block>
	+ tp_authority_discovery::AuthorityDiscoveryApi<Block>
where
	<Self as tp_api::ApiExt<Block>>::StateBackend: tp_api::StateBackend<BlakeTwo256>,
{}

impl<Api> RuntimeApiCollection for Api
where
	Api: tp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
		+ tp_api::ApiExt<Block, Error = tp_blockchain::Error>
		+ babe_primitives::BabeApi<Block>
		+ grandpa_primitives::GrandpaApi<Block>
		+ ParachainHost<Block>
		+ tp_block_builder::BlockBuilder<Block>
		+ fabric_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce>
		+ noble_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
		+ tp_api::Metadata<Block>
		+ tp_offchain::OffchainWorkerApi<Block>
		+ tp_session::SessionKeys<Block>
		+ tp_authority_discovery::AuthorityDiscoveryApi<Block>,
	<Self as tp_api::ApiExt<Block>>::StateBackend: tp_api::StateBackend<BlakeTwo256>,
{}

/// Trait that abstracts over all available client implementations.
///
/// For a concrete type there exists [`Client`].
pub trait AbstractClient<Block, Backend>:
	BlockchainEvents<Block> + Sized + Send + Sync
	+ ProvideRuntimeApi<Block>
	+ HeaderBackend<Block>
	+ CallApiAt<
		Block,
		Error = tp_blockchain::Error,
		StateBackend = Backend::State
	>
	where
		Block: BlockT,
		Backend: BackendT<Block>,
		Backend::State: tp_api::StateBackend<BlakeTwo256>,
		Self::Api: RuntimeApiCollection<StateBackend = Backend::State>,
{}

impl<Block, Backend, Client> AbstractClient<Block, Backend> for Client
	where
		Block: BlockT,
		Backend: BackendT<Block>,
		Backend::State: tp_api::StateBackend<BlakeTwo256>,
		Client: BlockchainEvents<Block> + ProvideRuntimeApi<Block> + HeaderBackend<Block>
			+ Sized + Send + Sync
			+ CallApiAt<
				Block,
				Error = tp_blockchain::Error,
				StateBackend = Backend::State
			>,
		Client::Api: RuntimeApiCollection<StateBackend = Backend::State>,
{}

/// Execute something with the client instance.
///
/// As there exist multiple chains inside Tetcoin, like Tetcoin itself, Kusama, Westend etc,
/// there can exist different kinds of client types. As these client types differ in the generics
/// that are being used, we can not easily return them from a function. For returning them from a
/// function there exists [`Client`]. However, the problem on how to use this client instance still
/// exists. This trait "solves" it in a dirty way. It requires a type to implement this trait and
/// than the [`execute_with_client`](ExecuteWithClient::execute_with_client) function can be called
/// with any possible client instance.
///
/// In a perfect world, we could make a closure work in this way.
pub trait ExecuteWithClient {
	/// The return type when calling this instance.
	type Output;

	/// Execute whatever should be executed with the given client instance.
	fn execute_with_client<Client, Api, Backend>(self, client: Arc<Client>) -> Self::Output
		where
			<Api as tp_api::ApiExt<Block>>::StateBackend: tp_api::StateBackend<BlakeTwo256>,
			Backend: tc_client_api::Backend<Block> + 'static,
			Backend::State: tp_api::StateBackend<BlakeTwo256>,
			Api: crate::RuntimeApiCollection<StateBackend = Backend::State>,
			Client: AbstractClient<Block, Backend, Api = Api> + 'static;
}

/// A handle to a Tetcoin client instance.
///
/// The Tetcoin service supports multiple different runtimes (Westend, Tetcoin itself, etc). As each runtime has a
/// specialized client, we need to hide them behind a trait. This is this trait.
///
/// When wanting to work with the inner client, you need to use `execute_with`.
///
/// See [`ExecuteWithClient`](trait.ExecuteWithClient.html) for more information.
pub trait ClientHandle {
	/// Execute the given something with the client.
	fn execute_with<T: ExecuteWithClient>(&self, t: T) -> T::Output;
}

/// A client instance of Tetcoin.
///
/// See [`ExecuteWithClient`] for more information.
#[derive(Clone)]
pub enum Client {
	Tetcoin(Arc<crate::FullClient<tetcoin_runtime::RuntimeApi, crate::TetcoinExecutor>>),
	Westend(Arc<crate::FullClient<westend_runtime::RuntimeApi, crate::WestendExecutor>>),
	Kusama(Arc<crate::FullClient<kusama_runtime::RuntimeApi, crate::KusamaExecutor>>),
	Rococo(Arc<crate::FullClient<rococo_runtime::RuntimeApi, crate::RococoExecutor>>),
}

impl ClientHandle for Client {
	fn execute_with<T: ExecuteWithClient>(&self, t: T) -> T::Output {
		match self {
			Self::Tetcoin(client) => {
				T::execute_with_client::<_, _, crate::FullBackend>(t, client.clone())
			},
			Self::Westend(client) => {
				T::execute_with_client::<_, _, crate::FullBackend>(t, client.clone())
			},
			Self::Kusama(client) => {
				T::execute_with_client::<_, _, crate::FullBackend>(t, client.clone())
			},
			Self::Rococo(client) => {
				T::execute_with_client::<_, _, crate::FullBackend>(t, client.clone())
			}
		}
	}
}

impl tc_client_api::UsageProvider<Block> for Client {
	fn usage_info(&self) -> tc_client_api::ClientInfo<Block> {
		match self {
			Self::Tetcoin(client) => client.usage_info(),
			Self::Westend(client) => client.usage_info(),
			Self::Kusama(client) => client.usage_info(),
			Self::Rococo(client) => client.usage_info(),
		}
	}
}

impl tc_client_api::BlockBackend<Block> for Client {
	fn block_body(
		&self,
		id: &BlockId<Block>
	) -> tp_blockchain::Result<Option<Vec<<Block as BlockT>::Extrinsic>>> {
		match self {
			Self::Tetcoin(client) => client.block_body(id),
			Self::Westend(client) => client.block_body(id),
			Self::Kusama(client) => client.block_body(id),
			Self::Rococo(client) => client.block_body(id),
		}
	}

	fn block(&self, id: &BlockId<Block>) -> tp_blockchain::Result<Option<SignedBlock<Block>>> {
		match self {
			Self::Tetcoin(client) => client.block(id),
			Self::Westend(client) => client.block(id),
			Self::Kusama(client) => client.block(id),
			Self::Rococo(client) => client.block(id),
		}
	}

	fn block_status(&self, id: &BlockId<Block>) -> tp_blockchain::Result<BlockStatus> {
		match self {
			Self::Tetcoin(client) => client.block_status(id),
			Self::Westend(client) => client.block_status(id),
			Self::Kusama(client) => client.block_status(id),
			Self::Rococo(client) => client.block_status(id),
		}
	}

	fn justification(
		&self,
		id: &BlockId<Block>
	) -> tp_blockchain::Result<Option<Justification>> {
		match self {
			Self::Tetcoin(client) => client.justification(id),
			Self::Westend(client) => client.justification(id),
			Self::Kusama(client) => client.justification(id),
			Self::Rococo(client) => client.justification(id),
		}
	}

	fn block_hash(
		&self,
		number: NumberFor<Block>
	) -> tp_blockchain::Result<Option<<Block as BlockT>::Hash>> {
		match self {
			Self::Tetcoin(client) => client.block_hash(number),
			Self::Westend(client) => client.block_hash(number),
			Self::Kusama(client) => client.block_hash(number),
			Self::Rococo(client) => client.block_hash(number),
		}
	}
}

impl tc_client_api::StorageProvider<Block, crate::FullBackend> for Client {
	fn storage(
		&self,
		id: &BlockId<Block>,
		key: &StorageKey,
	) -> tp_blockchain::Result<Option<StorageData>> {
		match self {
			Self::Tetcoin(client) => client.storage(id, key),
			Self::Westend(client) => client.storage(id, key),
			Self::Kusama(client) => client.storage(id, key),
			Self::Rococo(client) => client.storage(id, key),
		}
	}

	fn storage_keys(
		&self,
		id: &BlockId<Block>,
		key_prefix: &StorageKey,
	) -> tp_blockchain::Result<Vec<StorageKey>> {
		match self {
			Self::Tetcoin(client) => client.storage_keys(id, key_prefix),
			Self::Westend(client) => client.storage_keys(id, key_prefix),
			Self::Kusama(client) => client.storage_keys(id, key_prefix),
			Self::Rococo(client) => client.storage_keys(id, key_prefix),
		}
	}

	fn storage_hash(
		&self,
		id: &BlockId<Block>,
		key: &StorageKey,
	) -> tp_blockchain::Result<Option<<Block as BlockT>::Hash>> {
		match self {
			Self::Tetcoin(client) => client.storage_hash(id, key),
			Self::Westend(client) => client.storage_hash(id, key),
			Self::Kusama(client) => client.storage_hash(id, key),
			Self::Rococo(client) => client.storage_hash(id, key),
		}
	}

	fn storage_pairs(
		&self,
		id: &BlockId<Block>,
		key_prefix: &StorageKey,
	) -> tp_blockchain::Result<Vec<(StorageKey, StorageData)>> {
		match self {
			Self::Tetcoin(client) => client.storage_pairs(id, key_prefix),
			Self::Westend(client) => client.storage_pairs(id, key_prefix),
			Self::Kusama(client) => client.storage_pairs(id, key_prefix),
			Self::Rococo(client) => client.storage_pairs(id, key_prefix),
		}
	}

	fn storage_keys_iter<'a>(
		&self,
		id: &BlockId<Block>,
		prefix: Option<&'a StorageKey>,
		start_key: Option<&StorageKey>,
	) -> tp_blockchain::Result<KeyIterator<'a, <crate::FullBackend as tc_client_api::Backend<Block>>::State, Block>> {
		match self {
			Self::Tetcoin(client) => client.storage_keys_iter(id, prefix, start_key),
			Self::Westend(client) => client.storage_keys_iter(id, prefix, start_key),
			Self::Kusama(client) => client.storage_keys_iter(id, prefix, start_key),
			Self::Rococo(client) => client.storage_keys_iter(id, prefix, start_key),
		}
	}

	fn child_storage(
		&self,
		id: &BlockId<Block>,
		child_info: &ChildInfo,
		key: &StorageKey,
	) -> tp_blockchain::Result<Option<StorageData>> {
		match self {
			Self::Tetcoin(client) => client.child_storage(id, child_info, key),
			Self::Westend(client) => client.child_storage(id, child_info, key),
			Self::Kusama(client) => client.child_storage(id, child_info, key),
			Self::Rococo(client) => client.child_storage(id, child_info, key),
		}
	}

	fn child_storage_keys(
		&self,
		id: &BlockId<Block>,
		child_info: &ChildInfo,
		key_prefix: &StorageKey,
	) -> tp_blockchain::Result<Vec<StorageKey>> {
		match self {
			Self::Tetcoin(client) => client.child_storage_keys(id, child_info, key_prefix),
			Self::Westend(client) => client.child_storage_keys(id, child_info, key_prefix),
			Self::Kusama(client) => client.child_storage_keys(id, child_info, key_prefix),
			Self::Rococo(client) => client.child_storage_keys(id, child_info, key_prefix),
		}
	}

	fn child_storage_hash(
		&self,
		id: &BlockId<Block>,
		child_info: &ChildInfo,
		key: &StorageKey,
	) -> tp_blockchain::Result<Option<<Block as BlockT>::Hash>> {
		match self {
			Self::Tetcoin(client) => client.child_storage_hash(id, child_info, key),
			Self::Westend(client) => client.child_storage_hash(id, child_info, key),
			Self::Kusama(client) => client.child_storage_hash(id, child_info, key),
			Self::Rococo(client) => client.child_storage_hash(id, child_info, key),
		}
	}

	fn max_key_changes_range(
		&self,
		first: NumberFor<Block>,
		last: BlockId<Block>,
	) -> tp_blockchain::Result<Option<(NumberFor<Block>, BlockId<Block>)>> {
		match self {
			Self::Tetcoin(client) => client.max_key_changes_range(first, last),
			Self::Westend(client) => client.max_key_changes_range(first, last),
			Self::Kusama(client) => client.max_key_changes_range(first, last),
			Self::Rococo(client) => client.max_key_changes_range(first, last),
		}
	}

	fn key_changes(
		&self,
		first: NumberFor<Block>,
		last: BlockId<Block>,
		storage_key: Option<&PrefixedStorageKey>,
		key: &StorageKey,
	) -> tp_blockchain::Result<Vec<(NumberFor<Block>, u32)>> {
		match self {
			Self::Tetcoin(client) => client.key_changes(first, last, storage_key, key),
			Self::Westend(client) => client.key_changes(first, last, storage_key, key),
			Self::Kusama(client) => client.key_changes(first, last, storage_key, key),
			Self::Rococo(client) => client.key_changes(first, last, storage_key, key),
		}
	}
}

impl tp_blockchain::HeaderBackend<Block> for Client {
	fn header(&self, id: BlockId<Block>) -> tp_blockchain::Result<Option<Header>> {
		match self {
			Self::Tetcoin(client) => client.header(&id),
			Self::Westend(client) => client.header(&id),
			Self::Kusama(client) => client.header(&id),
			Self::Rococo(client) => client.header(&id),
		}
	}

	fn info(&self) -> tp_blockchain::Info<Block> {
		match self {
			Self::Tetcoin(client) => client.info(),
			Self::Westend(client) => client.info(),
			Self::Kusama(client) => client.info(),
			Self::Rococo(client) => client.info(),
		}
	}

	fn status(&self, id: BlockId<Block>) -> tp_blockchain::Result<tp_blockchain::BlockStatus> {
		match self {
			Self::Tetcoin(client) => client.status(id),
			Self::Westend(client) => client.status(id),
			Self::Kusama(client) => client.status(id),
			Self::Rococo(client) => client.status(id),
		}
	}

	fn number(&self, hash: Hash) -> tp_blockchain::Result<Option<BlockNumber>> {
		match self {
			Self::Tetcoin(client) => client.number(hash),
			Self::Westend(client) => client.number(hash),
			Self::Kusama(client) => client.number(hash),
			Self::Rococo(client) => client.number(hash),
		}
	}

	fn hash(&self, number: BlockNumber) -> tp_blockchain::Result<Option<Hash>> {
		match self {
			Self::Tetcoin(client) => client.hash(number),
			Self::Westend(client) => client.hash(number),
			Self::Kusama(client) => client.hash(number),
			Self::Rococo(client) => client.hash(number),
		}
	}
}
