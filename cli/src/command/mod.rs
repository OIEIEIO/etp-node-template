// TODO: https://github.com/paritytech/substrate/issues/5936
// mod config;

// --- crates ---
use log::info;
// --- substrate ---
use sc_cli::SubstrateCli;
use sc_executor::NativeExecutionDispatch;
// --- DNA ---
use crate::cli::{Cli, Subcommand};
// TODO: https://github.com/paritytech/substrate/issues/5936
// use config::Configuration;
use DNA_service::{dna-network_runtime, IdentifyVariant};

impl SubstrateCli for Cli {
	fn impl_name() -> &'static str {
		"dna-network"
	}

	fn impl_version() -> &'static str {
		env!("SUBSTRATE_CLI_IMPL_VERSION")
	}

	fn executable_name() -> &'static str {
		"DNA"
	}

	fn description() -> &'static str {
		env!("CARGO_PKG_DESCRIPTION")
	}

	fn author() -> &'static str {
		env!("CARGO_PKG_AUTHORS")
	}

	fn support_url() -> &'static str {
		"https://github.com/DNA-network/DNA/issues/new"
	}

	fn copyright_start_year() -> i32 {
		2018
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, Stetp> {
		Ok(match id {
			"dna-network-dev" | "dev" => Box::new(DNA_service::chain_spec::dna-network_development_config()),
			"dna-network-local" => Box::new(DNA_service::chain_spec::dna-network_local_testnet_config()),
			"dna-network-genesis" => Box::new(DNA_service::chain_spec::dna-network_build_spec_config()),
			"dna-network" | "" => Box::new(DNA_service::chain_spec::dna-network_config()?),
			path => Box::new(DNA_service::dna-networkChainSpec::from_json_file(
				std::path::PathBuf::from(path),
			)?),
		})
	}
}

/// Parses DNA specific CLI arguments and run the service.
pub fn run() -> sc_cli::Result<()> {
	let cli = Cli::from_args();
	match &cli.subcommand {
		None => {
			let mut runtime = cli.create_runner(&cli.run.base)?;
			let config = runtime.config_mut();

			// TODO: https://github.com/paritytech/substrate/issues/5936
			// if let Some(path) = &cli.conf {
			// 	if path.is_file() {
			// 		let DNA_config = Configuration::load_config(path);
			// 		DNA_config.update_config(config);
			// 	}
			// }

			info!("  _____                      _       _       ");
			info!(" |  __ \\                    (_)     (_)      ");
			info!(" | |  | | __ _ _ ____      ___ _ __  _  __ _ ");
			info!(" | |  | |/ _` | '__\\ \\ /\\ / / | '_ \\| |/ _` |");
			info!(" | |__| | (_| | |   \\ V  V /| | | | | | (_| |");
			info!(" |_____/ \\__,_|_|    \\_/\\_/ |_|_| |_|_|\\__,_|");

			if config.chain_spec.is_dna-network() {
				runtime.run_node(
					|config| DNA_service::dna-network_new_light(config),
					|config| DNA_service::dna-network_new_full(config),
					DNA_service::dna-networkExecutor::native_version().runtime_version,
				)
			} else {
				runtime.run_node(
					|config| DNA_service::dna-network_new_light(config),
					|config| DNA_service::dna-network_new_full(config),
					DNA_service::dna-networkExecutor::native_version().runtime_version,
				)
			}
		}
		Some(Subcommand::Base(subcommand)) => {
			let runtime = cli.create_runner(subcommand)?;

			if runtime.config().chain_spec.is_dna-network() {
				runtime.run_subcommand(subcommand, |config| {
					DNA_service::new_chain_ops::<
						dna-network_runtime::RuntimeApi,
						DNA_service::dna-networkExecutor,
						dna-network_runtime::UncheckedExtrinsic,
					>(config)
				})
			} else {
				runtime.run_subcommand(subcommand, |config| {
					DNA_service::new_chain_ops::<
						dna-network_runtime::RuntimeApi,
						DNA_service::dna-networkExecutor,
						dna-network_runtime::UncheckedExtrinsic,
					>(config)
				})
			}
		} // TODO: benchmark
		  // Some(Subcommand::Benchmark(cmd)) => {
		  // 	cmd.init(&version)?;
		  // 	cmd.update_config(&mut config, |id| load_spec(id), &version)?;
		  // 	cmd.run::<DNA_service::dna-network_runtime::Block, DNA_service::dna-networkExecutor>(config)
		  // }
	}
}
