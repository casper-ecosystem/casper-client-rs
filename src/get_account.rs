use std::str;

use async_trait::async_trait;
use clap::{ArgMatches, Command};

use casper_client::cli::CliError;

use crate::{command::ClientCommand, common, Success};

/// Legacy name of command.
const COMMAND_ALIAS: &str = "get-account-info";
const ACCOUNT_IDENTIFIER_IS_REQUIRED: bool = true;
const ACCOUNT_IDENTIFIER_ALIAS: &str = "public-key";
const ACCOUNT_IDENTIFIER_SHORT_ALIAS: char = 'p';

pub struct GetAccount;

/// This struct defines the order in which the args are shown for this subcommand's help message.
enum DisplayOrder {
    Verbose,
    NodeAddress,
    RpcId,
    BlockIdentifier,
    AccountIdentifier,
}

#[async_trait]
impl ClientCommand for GetAccount {
    const NAME: &'static str = "get-account";
    const ABOUT: &'static str = "Retrieve account information from the network";

    fn build(display_order: usize) -> Command {
        Command::new(Self::NAME)
            .alias(COMMAND_ALIAS)
            .about(Self::ABOUT)
            .display_order(display_order)
            .arg(common::verbose::arg(DisplayOrder::Verbose as usize))
            .arg(common::node_address::arg(
                DisplayOrder::NodeAddress as usize,
            ))
            .arg(common::rpc_id::arg(DisplayOrder::RpcId as usize))
            .arg(common::block_identifier::arg(
                DisplayOrder::BlockIdentifier as usize,
                true,
            ))
            .arg(
                common::account_identifier::arg(
                    DisplayOrder::AccountIdentifier as usize,
                    ACCOUNT_IDENTIFIER_IS_REQUIRED,
                )
                .alias(ACCOUNT_IDENTIFIER_ALIAS)
                .short_alias(ACCOUNT_IDENTIFIER_SHORT_ALIAS),
            )
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        let maybe_rpc_id = common::rpc_id::get(matches);
        let node_address = common::node_address::get(matches);
        let verbosity_level = common::verbose::get(matches);
        let block_identifier = common::block_identifier::get(matches);
        let account_idenfitier = common::account_identifier::get(matches)?;

        casper_client::cli::get_account(
            maybe_rpc_id,
            node_address,
            verbosity_level,
            block_identifier,
            &account_idenfitier,
        )
        .await
        .map(Success::from)
    }
}
