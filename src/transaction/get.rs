use std::str;

use async_trait::async_trait;
use clap::{Arg, ArgAction, ArgMatches, Command};

use casper_client::cli::CliError;

use crate::{command::ClientCommand, common, Success};

pub struct GetTransaction;

/// This struct defines the order in which the args are shown for this subcommand's help message.
enum DisplayOrder {
    Verbose,
    NodeAddress,
    RpcId,
    TransactionHash,
    FinalizedApprovals,
}

const ALIAS: &str = "get-txn";
/// Handles providing the arg for and retrieval of the transaction hash.
mod transaction_hash {
    use super::*;

    const ARG_NAME: &str = "transaction-hash";
    const ARG_VALUE_NAME: &str = "HEX STRING";
    const ARG_HELP: &str = "Hex-encoded transaction hash";

    pub(super) fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .required(true)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::TransactionHash as usize)
    }

    pub(super) fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_else(|| panic!("should have {} arg", ARG_NAME))
    }
}

/// Handles providing the arg for the retrieval of the finalized approvals.
mod finalized_approvals {
    use super::*;

    const ARG_NAME: &str = "get-finalized-approvals";
    const ARG_SHORT: char = 'a';
    const ARG_HELP: &str =
        "If passed, the returned transaction approvals are the ones finalized in the block.\
         Otherwise the approvals attached to the transaction when first received by the node \
         will be returned";

    pub(super) fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .short(ARG_SHORT)
            .required(false)
            .action(ArgAction::SetTrue)
            .help(ARG_HELP)
            .display_order(DisplayOrder::FinalizedApprovals as usize)
    }

    pub(super) fn get(matches: &ArgMatches) -> bool {
        matches
            .get_one::<bool>(ARG_NAME)
            .copied()
            .unwrap_or_default()
    }
}

#[async_trait]
impl ClientCommand for GetTransaction {
    const NAME: &'static str = "get-transaction";
    const ABOUT: &'static str = "Retrieve a transaction from the network";

    fn build(display_order: usize) -> Command {
        Command::new(Self::NAME)
            .alias(ALIAS)
            .about(Self::ABOUT)
            .display_order(display_order)
            .arg(common::verbose::arg(DisplayOrder::Verbose as usize))
            .arg(common::node_address::arg(
                DisplayOrder::NodeAddress as usize,
            ))
            .arg(common::rpc_id::arg(DisplayOrder::RpcId as usize))
            .arg(transaction_hash::arg())
            .arg(finalized_approvals::arg())
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        let maybe_rpc_id = common::rpc_id::get(matches);
        let node_address = common::node_address::get(matches);
        let verbosity_level = common::verbose::get(matches);
        let transaction_hash = transaction_hash::get(matches);
        let finalized_approvals = finalized_approvals::get(matches);

        casper_client::cli::get_transaction(
            maybe_rpc_id,
            node_address,
            verbosity_level,
            transaction_hash,
            finalized_approvals,
        )
        .await
        .map(Success::from)
    }
}
