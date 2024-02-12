use async_trait::async_trait;
use clap::{ArgMatches, Command};

use casper_client::cli::CliError;

use super::creation_common;
use crate::{command::ClientCommand, common, Success};

pub struct SignTransaction;

#[async_trait]
impl ClientCommand for SignTransaction {
    const NAME: &'static str = "sign-transaction";
    const ABOUT: &'static str =
        "Read a previously-saved transaction from a file, cryptographically sign it, and output it to a \
        file or stdout";

    fn build(display_order: usize) -> Command {
        Command::new(Self::NAME)
            .about(Self::ABOUT)
            .display_order(display_order)
            .arg(
                common::secret_key::arg(creation_common::DisplayOrder::SecretKey as usize, "")
                    .required(true),
            )
            .arg(creation_common::transaction_path::arg())
            .arg(creation_common::output::arg())
            .arg(common::force::arg(
                creation_common::DisplayOrder::Force as usize,
                true,
            ))
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        let input_path = creation_common::transaction_path::get(matches);
        let secret_key = common::secret_key::get(matches).unwrap_or_default();
        let maybe_output_path = creation_common::output::get(matches).unwrap_or_default();
        let force = common::force::get(matches);
        let output = if maybe_output_path.is_empty(){
            String::new()
        } else {
            format!(
                "Signed the transaction at {} and wrote to {}",
                input_path, maybe_output_path
            )
        };

        // convert maybe_output_path to an instance of Option to change the function signature of sign_transaction_file
        let maybe_output_path = if maybe_output_path.is_empty() {
            None
        } else {
            Some(maybe_output_path)
        };

        casper_client::cli::sign_transaction_file(input_path, secret_key, maybe_output_path, force)
            .map(|_| {
                Success::Output(output)
            })
    }
}
