use clap::{ArgMatches, Command};

use async_trait::async_trait;

use casper_client::cli::CliError;

use super::creation_common::{self, DisplayOrder};
use crate::{command::ClientCommand, common, Success};

pub struct SendTransaction;

const ALIAS: &str = "send-txn";

#[async_trait]
impl ClientCommand for SendTransaction {
    const NAME: &'static str = "send-transaction";
    const ABOUT: &'static str =
        "Read a previously-saved deploy from a file and send it to the network for execution";

    fn build(display_order: usize) -> Command {
        Command::new(Self::NAME)
            .about(Self::ABOUT)
            .display_order(display_order)
            .alias(ALIAS)
            .arg(common::verbose::arg(DisplayOrder::Verbose as usize))
            .arg(common::node_address::arg(
                DisplayOrder::NodeAddress as usize,
            ))
            .arg(common::rpc_id::arg(DisplayOrder::RpcId as usize))
            .arg(creation_common::speculative_exec::arg())
            .arg(creation_common::transaction_path::arg())
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        let maybe_speculative_exec = creation_common::speculative_exec::get(matches);
        let maybe_rpc_id = common::rpc_id::get(matches);
        let node_address = common::node_address::get(matches);
        let verbosity_level = common::verbose::get(matches);
        let input_path = creation_common::transaction_path::get(matches).unwrap_or_default();
        if input_path.is_empty() {
            return Err(CliError::InvalidArgument {
                context: "send_deploy",
                error: "Transaction path cannot be empty".to_string(),
            });
        }

        if let Some(speculative_exec_height_identifier) = maybe_speculative_exec {
            casper_client::cli::speculative_send_transaction_file(
                maybe_rpc_id,
                node_address,
                verbosity_level,
                input_path,
                speculative_exec_height_identifier,
            )
            .await
            .map(Success::from)
        } else {
            casper_client::cli::send_transaction_file(
                maybe_rpc_id,
                node_address,
                verbosity_level,
                input_path,
            )
            .await
            .map(Success::from)
        }
    }
}
