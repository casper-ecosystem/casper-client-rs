use async_trait::async_trait;
use clap::{ArgMatches, Command};

use casper_client::cli::{CliError, DeployStrParams};

use super::creation_common::{self, DisplayOrder};
use crate::{command::ClientCommand, common, Success};

pub struct PutDeploy;

#[async_trait]
impl ClientCommand for PutDeploy {
    const NAME: &'static str = "put-deploy";
    const ABOUT: &'static str = "Create a deploy and send it to the network for execution";

    fn build(display_order: usize) -> Command {
        let subcommand = Command::new(Self::NAME)
            .about(Self::ABOUT)
            .display_order(display_order)
            .arg(common::verbose::arg(DisplayOrder::Verbose as usize))
            .arg(common::rpc_id::arg(DisplayOrder::RpcId as usize))
            .arg(creation_common::speculative_exec::arg())
            .arg(creation_common::gas_price::arg());
        let subcommand = creation_common::apply_common_session_options(subcommand);
        let subcommand = creation_common::apply_common_payment_options(subcommand, None);
        creation_common::apply_common_creation_options(subcommand, true, true)
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        creation_common::show_simple_arg_examples_and_exit_if_required(matches);
        creation_common::show_json_args_examples_and_exit_if_required(matches);

        let maybe_rpc_id = common::rpc_id::get(matches);
        let node_address = common::node_address::get(matches);
        let verbosity_level = common::verbose::get(matches);

        let secret_key = common::secret_key::get(matches).unwrap_or_default();
        let maybe_speculative_exec = creation_common::speculative_exec::get(matches);
        let timestamp = creation_common::timestamp::get(matches);
        let ttl = creation_common::ttl::get(matches);
        let chain_name = creation_common::chain_name::get(matches);
        let session_account = creation_common::session_account::get(matches)?;
        let gas_price = creation_common::gas_price::get(matches);
        let session_str_params = creation_common::session_str_params(matches);
        let payment_str_params = creation_common::payment_str_params(matches);

        if let Some(speculative_exec) = maybe_speculative_exec {
            casper_client::cli::speculative_put_deploy(
                speculative_exec,
                maybe_rpc_id,
                node_address,
                verbosity_level,
                DeployStrParams {
                    secret_key,
                    timestamp,
                    ttl,
                    chain_name,
                    session_account: &session_account,
                    gas_price_tolerance: gas_price,
                },
                session_str_params,
                payment_str_params,
            )
            .await
            .map(Success::from)
        } else {
            casper_client::cli::put_deploy(
                maybe_rpc_id,
                node_address,
                verbosity_level,
                DeployStrParams {
                    secret_key,
                    timestamp,
                    ttl,
                    chain_name,
                    session_account: &session_account,
                    gas_price_tolerance: gas_price,
                },
                session_str_params,
                payment_str_params,
            )
            .await
            .map(Success::from)
        }
    }
}
