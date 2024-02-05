use std::str;

use async_trait::async_trait;
use clap::{ArgMatches, Command};

use casper_client::cli::CliError;

use crate::{command::ClientCommand, common, Success};

const ENTITY_IDENTIFIER_IS_REQUIRED: bool = true;
const PUBLIC_KEY_IDENTIFIER_ALIAS: &str = "public-key";
const PUBLIC_KEY_IDENTIFIER_SHORT_ALIAS: char = 'p';
const ACCOUNT_HASH_IDENTIFIER_ALIAS: &str = "account-hash";
const ACCOUNT_HASH_IDENTIFIER_SHORT_ALIAS: char = 'a';

pub struct GetEntity;

/// This struct defines the order in which the args are shown for this subcommand's help message.
enum DisplayOrder {
    Verbose,
    NodeAddress,
    RpcId,
    BlockIdentifier,
    EntityIdentifier,
}

#[async_trait]
impl ClientCommand for GetEntity {
    const NAME: &'static str = "get-entity";
    const ABOUT: &'static str = "Retrieve information for an addressable entity from the network";

    fn build(display_order: usize) -> Command {
        Command::new(Self::NAME)
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
                common::entity_identifier::arg(
                    DisplayOrder::EntityIdentifier as usize,
                    ENTITY_IDENTIFIER_IS_REQUIRED,
                )
                .alias(PUBLIC_KEY_IDENTIFIER_ALIAS)
                .short_alias(PUBLIC_KEY_IDENTIFIER_SHORT_ALIAS)
                .alias(ACCOUNT_HASH_IDENTIFIER_ALIAS)
                .short_alias(ACCOUNT_HASH_IDENTIFIER_SHORT_ALIAS),
            )
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        let maybe_rpc_id = common::rpc_id::get(matches);
        let node_address = common::node_address::get(matches);
        let verbosity_level = common::verbose::get(matches);
        let block_identifier = common::block_identifier::get(matches);
        let entity_idenfitier = common::entity_identifier::get(matches)?;

        casper_client::cli::get_entity(
            maybe_rpc_id,
            node_address,
            verbosity_level,
            block_identifier,
            &entity_idenfitier,
        )
        .await
        .map(Success::from)
    }
}
