use std::str;

use async_trait::async_trait;
use casper_types::{AsymmetricType, PublicKey};
use clap::{ArgMatches, Command};

use casper_client::cli::CliError;

use crate::{
    command::ClientCommand,
    common::{self},
    Success,
};

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

    const ARG_NAME: &str = "verification-url-basepath";
    const ARG_SHORT: char = 'u';
    const ARG_VALUE_NAME: &str = "HOST:PORT";
    const ARG_DEFAULT: &str = "http://localhost:8080";
    const ARG_HELP: &str = "Hostname or IP and port of the verification API";

    pub fn arg(order: usize) -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .short(ARG_SHORT)
            .required(false)
            .default_value(ARG_DEFAULT)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(order)
    }

    pub fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_else(|| panic!("should have {} arg", ARG_NAME))
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
                true,
            ))
            .arg(verification_url_base_path::arg(
                DisplayOrder::VerificationUrlBasePath as usize,
            ))
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        let deploy_hash = common::deploy_hash::get(matches);
        let hex_public_key = common::public_key::get(matches, true)?;
        let public_key = PublicKey::from_hex(&hex_public_key).map_err(|error| {
            eprintln!("Can't parse {} as a public key: {}", hex_public_key, error);
            CliError::FailedToParsePublicKey {
                context: "account-address".to_string(),
                error,
            }
        })?;
        let verbosity_level = common::verbose::get(matches);
        let verification_url_base_path = verification_url_base_path::get(matches);

        casper_client::cli::verify_contract(
            deploy_hash,
            public_key,
            verbosity_level,
            verification_url_base_path,
        )
        .await
        .map(Success::from)
    }
}
