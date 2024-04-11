use std::str;

use async_trait::async_trait;
use casper_client::cli::CliError;
use clap::{ArgMatches, Command};

use crate::{command::ClientCommand, common, Success};

pub struct VerifyContract;

/// This struct defines the order in which the args are shown for this subcommand's help message.
enum DisplayOrder {
    Verbose,
    DeployHash,
    VerificationUrlBasePath,
    VerificationProjectPath,
}

mod verification_url_base_path {
    use clap::{Arg, ArgMatches};

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

mod verification_project_path {
    use clap::{Arg, ArgMatches};

    static ARG_NAME: &str = "verification-source-code-path";

    pub fn arg(order: usize) -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .short('p')
            .required(false)
            .value_name("PATH")
            .help("Source code path")
            .display_order(order)
    }

    pub fn get(matches: &ArgMatches) -> Option<&str> {
        matches.get_one::<String>(ARG_NAME).map(String::as_str)
    }
}

#[async_trait]
impl ClientCommand for VerifyContract {
    const NAME: &'static str = "verify-contract";
    const ABOUT: &'static str =
        "Verifies a smart contracts source code using verification service. \
        The source code will be uploaded, built, and compared against the deployed contract binary. \
        You may specify a path from which the code will be read and compressed from, or omit the path. \
        If the path is omitted, the archive will be built from the current working directory.";

    fn build(display_order: usize) -> Command {
        Command::new(Self::NAME)
            .about(Self::ABOUT)
            .display_order(display_order)
            .arg(common::verbose::arg(DisplayOrder::Verbose as usize))
            .arg(common::deploy_hash::arg(DisplayOrder::DeployHash as usize))
            .arg(verification_url_base_path::arg(
                DisplayOrder::VerificationUrlBasePath as usize,
            ))
            .arg(verification_project_path::arg(
                DisplayOrder::VerificationProjectPath as usize,
            ))
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        let deploy_hash = common::deploy_hash::get(matches);
        let verification_url_base_path = verification_url_base_path::get(matches);
        let verification_project_path = verification_project_path::get(matches);
        let verbosity_level = common::verbose::get(matches);

        casper_client::cli::verify_contract(
            deploy_hash,
            verification_url_base_path,
            verification_project_path,
            verbosity_level,
        )
        .await
        .map(Success::from)
    }
}
