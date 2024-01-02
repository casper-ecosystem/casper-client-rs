use std::str;

use async_trait::async_trait;
use casper_types::{AsymmetricType, PublicKey};
use clap::{ArgMatches, Command};

use casper_client::cli::CliError;

use crate::{
    command::ClientCommand,
    common::{self, block_identifier},
    Success,
};

pub struct VerifyContract;

/// This struct defines the order in which the args are shown for this subcommand's help message.
enum DisplayOrder {
    Verbose,
    BlockIdentifier,
    PublicKey,
}

#[async_trait]
impl ClientCommand for VerifyContract {
    const NAME: &'static str = "verify-contract";
    const ABOUT: &'static str = "Verify smart contract source code";

    fn build(display_order: usize) -> Command {
        Command::new(Self::NAME)
            .about(Self::ABOUT)
            .display_order(display_order)
            .arg(common::verbose::arg(DisplayOrder::Verbose as usize))
            .arg(common::block_identifier::arg(
                DisplayOrder::BlockIdentifier as usize,
                true,
            ))
            .arg(common::public_key::arg(
                DisplayOrder::PublicKey as usize,
                true,
            ))
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        let block_identifier = common::block_identifier::get(matches);
        let hex_public_key = common::public_key::get(matches, true)?;
        let public_key = PublicKey::from_hex(&hex_public_key).map_err(|error| {
            eprintln!("Can't parse {} as a public key: {}", hex_public_key, error);
            CliError::FailedToParsePublicKey {
                context: "account-address".to_string(),
                error,
            }
        })?;
        let verbosity_level = common::verbose::get(matches);

        casper_client::cli::verify_contract(block_identifier, public_key, verbosity_level)
            .await
            .map(Success::from)
    }
}
