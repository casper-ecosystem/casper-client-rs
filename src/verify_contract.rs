use std::str;

use async_trait::async_trait;
use casper_client::cli::CliError;
use clap::{ArgMatches, Command};

use crate::{command::ClientCommand, common, Success};

use clap::Arg;

pub struct VerifyContract;

/// This struct defines the order in which the args are shown for this subcommand's help message.
enum DisplayOrder {
    Verbose,
    DeployHash,
    PublicKey,
    VerificationUrlBasePath,
}

pub mod verification_url_base_path {
    use super::*;

    static ARG_NAME: &str = "verification-url-basepath";

    pub fn arg(order: usize) -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .short('u')
            .required(false)
            .default_value("http://localhost:8080")
            .value_name("HOST:PORT")
            .help("Hostname or IP and port of the verification API")
            .display_order(order)
    }

    pub fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_else(|| panic!("should have {ARG_NAME} arg"))
    }
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
            .arg(common::deploy_hash::arg(DisplayOrder::DeployHash as usize))
            .arg(common::public_key::arg(
                DisplayOrder::PublicKey as usize,
                true,
            ))
            .arg(verification_url_base_path::arg(
                DisplayOrder::VerificationUrlBasePath as usize,
            ))
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        let deploy_hash = common::deploy_hash::get(matches);
        let verification_url_base_path = verification_url_base_path::get(matches);
        let verbosity_level = common::verbose::get(matches);

        casper_client::cli::verify_contract(
            deploy_hash,
            verification_url_base_path,
            verbosity_level,
        )
        .await
        .map(Success::from)
    }
}
