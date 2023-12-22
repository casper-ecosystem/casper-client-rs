use std::str;

use async_trait::async_trait;
use clap::{ArgMatches, Command};

use casper_client::cli::CliError;

use crate::{command::ClientCommand, common, Success};

pub struct VerifyContract;

/// This struct defines the order in which the args are shown for this subcommand's help message.
enum DisplayOrder {
    Verbose,
    BlockIdentifier,
    PublicKey
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
            .arg(common::block_identifier::arg(DisplayOrder::BlockIdentifier as usize, true))
            .arg(common::public_key::arg(DisplayOrder::PublicKey as usize, true))

    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        Ok(Success::Output("OK".to_string()))
    }
}
