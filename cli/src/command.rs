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

use log::info;
use service::{IdentifyVariant, self};
use tc_cli::{TetcoreCli, RuntimeVersion, Role};
use crate::cli::{Cli, Subcommand};
use futures::future::TryFutureExt;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error(transparent)]
	TetcoinService(#[from] service::Error),

	#[error(transparent)]
	TetcoreCli(#[from] tc_cli::Error),

	#[error(transparent)]
	TetcoreService(#[from] tc_service::Error),

	#[error("Other: {0}")]
	Other(String),
}

impl std::convert::From<String> for Error {
	fn from(s: String) -> Self {
		Self::Other(s)
	}
}

type Result<T> = std::result::Result<T, Error>;

fn get_exec_name() -> Option<String> {
	std::env::current_exe()
		.ok()
		.and_then(|pb| pb.file_name().map(|s| s.to_os_string()))
		.and_then(|s| s.into_string().ok())
}

impl TetcoreCli for Cli {
	fn impl_name() -> String { "Parity Tetcoin".into() }

	fn impl_version() -> String { env!("TETCORE_CLI_IMPL_VERSION").into() }

	fn description() -> String { env!("CARGO_PKG_DESCRIPTION").into() }

	fn author() -> String { env!("CARGO_PKG_AUTHORS").into() }

	fn support_url() -> String { "https://github.com/tetcoin/tetcoin/issues/new".into() }

	fn copyright_start_year() -> i32 { 2017 }

	fn executable_name() -> String { "tetcoin".into() }

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn tc_service::ChainSpec>, String> {
		let id = if id == "" {
			let n = get_exec_name().unwrap_or_default();
			["tetcoin", "metrocoin", "westend", "rococo"].iter()
				.cloned()
				.find(|&chain| n.starts_with(chain))
				.unwrap_or("tetcoin")
		} else { id };
		Ok(match id {
			"tetcoin-dev" | "dev" => Box::new(service::chain_spec::tetcoin_development_config()?),
			"tetcoin-local" => Box::new(service::chain_spec::tetcoin_local_testnet_config()?),
			"tetcoin-staging" => Box::new(service::chain_spec::tetcoin_staging_testnet_config()?),
			"metrocoin-dev" => Box::new(service::chain_spec::metrocoin_development_config()?),
			"metrocoin-local" => Box::new(service::chain_spec::metrocoin_local_testnet_config()?),
			"metrocoin-staging" => Box::new(service::chain_spec::metrocoin_staging_testnet_config()?),
			"tetcoin" => Box::new(service::chain_spec::tetcoin_config()?),
			"westend" => Box::new(service::chain_spec::westend_config()?),
			"metrocoin" => Box::new(service::chain_spec::metrocoin_config()?),
			"westend-dev" => Box::new(service::chain_spec::westend_development_config()?),
			"westend-local" => Box::new(service::chain_spec::westend_local_testnet_config()?),
			"westend-staging" => Box::new(service::chain_spec::westend_staging_testnet_config()?),
			"rococo-staging" => Box::new(service::chain_spec::rococo_staging_testnet_config()?),
			"rococo-local" => Box::new(service::chain_spec::rococo_local_testnet_config()?),
			"rococo" => Box::new(service::chain_spec::rococo_config()?),
			path => {
				let path = std::path::PathBuf::from(path);

				let starts_with = |prefix: &str| {
					path.file_name().map(|f| f.to_str().map(|s| s.starts_with(&prefix))).flatten().unwrap_or(false)
				};

				// When `force_*` is given or the file name starts with the name of one of the known chains,
				// we use the chain spec for the specific chain.
				if self.run.force_rococo || starts_with("rococo") {
					Box::new(service::RococoChainSpec::from_json_file(path)?)
				} else if self.run.force_metrocoin || starts_with("metrocoin") {
					Box::new(service::MetrocoinChainSpec::from_json_file(path)?)
				} else if self.run.force_westend || starts_with("westend") {
					Box::new(service::WestendChainSpec::from_json_file(path)?)
				} else {
					Box::new(service::TetcoinChainSpec::from_json_file(path)?)
				}
			},
		})
	}

	fn native_runtime_version(spec: &Box<dyn service::ChainSpec>) -> &'static RuntimeVersion {
		if spec.is_metrocoin() {
			&service::metrocoin_runtime::VERSION
		} else if spec.is_westend() {
			&service::westend_runtime::VERSION
		} else if spec.is_rococo() {
			&service::rococo_runtime::VERSION
		} else {
			&service::tetcoin_runtime::VERSION
		}
	}
}

/// dust TODO: unrevert Kusama to MetrocoinAccount and the same for Polkadot
fn set_default_ss58_version(spec: &Box<dyn service::ChainSpec>) {
	use tet_core::crypto::Ss58AddressFormat;

	let ss58_version = if spec.is_metrocoin() {
		Ss58AddressFormat::KusamaAccount
	} else if spec.is_westend() {
		Ss58AddressFormat::TetcoreAccount
	} else {
		Ss58AddressFormat::PolkadotAccount
	};

	tet_core::crypto::set_default_ss58_version(ss58_version);
}

/// Parses tetcoin specific CLI arguments and run the service.
pub fn run() -> Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		None => {
			let runner = cli.create_runner(&cli.run.base)
				.map_err(Error::from)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			let grandpa_pause = if cli.run.grandpa_pause.is_empty() {
				None
			} else {
				Some((cli.run.grandpa_pause[0], cli.run.grandpa_pause[1]))
			};

			if chain_spec.is_metrocoin() {
				info!("----------------------------");
				info!("This chain is not in any way");
				info!("      endorsed by the       ");
				info!("   METROCOIN FOUNDATION     ");
				info!("----------------------------");
			}

			let jaeger_agent = cli.run.jaeger_agent;

			runner.run_node_until_exit(move |config| async move {
				let role = config.role.clone();

				let task_manager = match role {
					Role::Light => service::build_light(config).map(|(task_manager, _, _)| task_manager),
					_ => service::build_full(
						config,
						service::IsCollator::No,
						grandpa_pause,
						jaeger_agent,
					).map(|full| full.task_manager)
				}?;
				Ok::<_, Error>(task_manager)
			})
		},
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			Ok(runner.sync_run(|config| {
				cmd.run(config.chain_spec, config.network)
			})?)
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)
				.map_err(Error::TetcoreCli)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			runner.async_run(|mut config| {
				let (client, _, import_queue, task_manager) = service::new_chain_ops(&mut config, None)?;
				Ok((cmd.run(client, import_queue).map_err(Error::TetcoreCli), task_manager))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			Ok(runner.async_run(|mut config| {
				let (client, _, _, task_manager) = service::new_chain_ops(&mut config, None)
					.map_err(Error::TetcoinService)?;
				Ok((cmd.run(client, config.database).map_err(Error::TetcoreCli), task_manager))
			})?)
		},
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			Ok(runner.async_run(|mut config| {
				let (client, _, _, task_manager) = service::new_chain_ops(&mut config, None)?;
				Ok((cmd.run(client, config.chain_spec).map_err(Error::TetcoreCli), task_manager))
			})?)
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			Ok(runner.async_run(|mut config| {
				let (client, _, import_queue, task_manager) = service::new_chain_ops(&mut config, None)?;
				Ok((cmd.run(client, import_queue).map_err(Error::TetcoreCli), task_manager))
			})?)
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			Ok(runner.sync_run(|config| cmd.run(config.database))?)
		},
		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			Ok(runner.async_run(|mut config| {
				let (client, backend, _, task_manager) = service::new_chain_ops(&mut config, None)?;
				Ok((cmd.run(client, backend).map_err(Error::TetcoreCli),task_manager))
			})?)
		},
		Some(Subcommand::ValidationWorker(cmd)) => {
			let mut builder = tc_cli::LoggerBuilder::new("");
			builder.with_colors(false);
			let _ = builder.init();

			if cfg!(feature = "browser") || cfg!(target_os = "android") {
				Err(tc_cli::Error::Input("Cannot run validation worker in browser".into()).into())
			} else {
				#[cfg(not(any(target_os = "android", feature = "browser")))]
				tetcoin_parachain::wasm_executor::run_worker(&cmd.mem_id)?;
				Ok(())
			}
		},
		Some(Subcommand::Benchmark(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			Ok(runner.sync_run(|config| {
				cmd.run::<service::metrocoin_runtime::Block, service::MetrocoinExecutor>(config)
				.map_err(|e| Error::TetcoreCli(e))
			})?)
		},
		Some(Subcommand::Key(cmd)) => Ok(cmd.run(&cli)?),
	}?;
	Ok(())
}
