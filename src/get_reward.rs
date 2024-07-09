use std::{fs, str};

use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command};

use casper_client::cli::CliError;
use casper_types::PublicKey;

use crate::{command::ClientCommand, common, Success};

pub struct GetReward;

/// This struct defines the order in which the args are shown for this subcommand's help message.
enum DisplayOrder {
    Verbose,
    NodeAddress,
    RpcId,
    EraIdentifier,
    Validator,
    Delegator,
}

/// Handles providing the arg for and retrieval of public keys.
mod key {
    use casper_types::AsymmetricType;

    use super::*;

    const ARG_VALUE_NAME: &str = "FORMATTED STRING or PATH";

    pub(super) fn arg(arg_name: &'static str, arg_help: &'static str, display_order: usize) -> Arg {
        Arg::new(arg_name)
            .long(arg_name)
            .value_name(ARG_VALUE_NAME)
            .help(arg_help)
            .display_order(display_order)
    }

    pub(super) fn get(arg_name: &'static str, matches: &ArgMatches) -> Result<String, CliError> {
        let value = matches
            .get_one::<String>(arg_name)
            .map(String::as_str)
            .unwrap_or_default();

        // Try to read as a PublicKey PEM file first.
        if let Ok(public_key) = PublicKey::from_file(value) {
            return Ok(public_key.to_hex());
        }

        // Try to read as a hex-encoded PublicKey file next.
        if let Ok(hex_public_key) = fs::read_to_string(value) {
            let _ = PublicKey::from_hex(&hex_public_key).map_err(|error| {
                eprintln!(
                    "Can't parse the contents of {} as a public key: {}",
                    value, error
                );
                CliError::FailedToParsePublicKey {
                    context: "get dictionary item public key".to_string(),
                    error,
                }
            })?;
            return Ok(hex_public_key);
        }

        // Just return the value.
        Ok(value.to_string())
    }
}

mod validator {
    use super::*;

    pub(crate) const ARG_NAME: &str = "validator";
    const ARG_HELP: &str =
        "A public key of the validator, formatted as a hex-encoded string or a path to a file";

    pub(super) fn arg() -> Arg {
        key::arg(ARG_NAME, ARG_HELP, DisplayOrder::Validator as usize).required(true)
    }

    pub(super) fn get(matches: &ArgMatches) -> Result<String, CliError> {
        key::get(ARG_NAME, matches)
    }
}

mod delegator {
    use super::*;

    pub(crate) const ARG_NAME: &str = "delegator";
    const ARG_HELP: &str =
        "A public key of the delegator, formatted as a hex-encoded string or a path to a file";

    pub(super) fn arg() -> Arg {
        key::arg(ARG_NAME, ARG_HELP, DisplayOrder::Delegator as usize).required(false)
    }

    pub(super) fn get(matches: &ArgMatches) -> Result<String, CliError> {
        key::get(ARG_NAME, matches)
    }
}

#[async_trait]
impl ClientCommand for GetReward {
    const NAME: &'static str = "get-reward";
    const ABOUT: &'static str = "Retrieve information for a reward from the network";

    fn build(display_order: usize) -> Command {
        Command::new(Self::NAME)
            .about(Self::ABOUT)
            .display_order(display_order)
            .arg(common::verbose::arg(DisplayOrder::Verbose as usize))
            .arg(common::node_address::arg(
                DisplayOrder::NodeAddress as usize,
            ))
            .arg(common::rpc_id::arg(DisplayOrder::RpcId as usize))
            .arg(common::era_identifier::arg(
                DisplayOrder::EraIdentifier as usize,
                true,
            ))
            .arg(validator::arg())
            .arg(delegator::arg())
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        let maybe_rpc_id = common::rpc_id::get(matches);
        let node_address = common::node_address::get(matches);
        let verbosity_level = common::verbose::get(matches);
        let era_identifier = common::era_identifier::get(matches);
        let validator = validator::get(matches)?;
        let delegator = delegator::get(matches)?;

        casper_client::cli::get_reward(
            maybe_rpc_id,
            node_address,
            verbosity_level,
            era_identifier,
            &validator,
            &delegator,
        )
        .await
        .map(Success::from)
    }
}
