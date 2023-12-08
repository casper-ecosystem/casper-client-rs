use async_trait::async_trait;
use clap::{ArgMatches, Command};

use casper_client::cli::{CliError, TransactionStrParams};

use super::creation_common;
use crate::{command::ClientCommand, common, Success};

pub struct MakeTransaction;

#[async_trait]
impl ClientCommand for MakeTransaction {
    const NAME: &'static str = "make-transaction";
    const ABOUT: &'static str =
        "Create a transaction and output it to a file or stdout. As a file, the transaction can subsequently \
        be signed by other parties using the 'sign-transaction' subcommand and then sent to the network \
        for execution using the 'send-transaction' subcommand";

    fn build(display_order: usize) -> Command {
        let subcommand = Command::new(Self::NAME)
            .about(Self::ABOUT)
            .arg(creation_common::output::arg())
            .arg(common::force::arg(
                creation_common::DisplayOrder::Force as usize,
                true,
            ))
            .display_order(display_order);
        let subcommand = creation_common::apply_common_session_options(subcommand);
        let subcommand = creation_common::apply_common_payment_options(subcommand);
        creation_common::apply_common_creation_options(subcommand, false, false)
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        creation_common::show_simple_arg_examples_and_exit_if_required(matches);
        creation_common::show_json_args_examples_and_exit_if_required(matches);

        let secret_key = common::secret_key::get(matches).unwrap_or_default();
        let timestamp = creation_common::timestamp::get(matches);
        let ttl = creation_common::ttl::get(matches);
        let chain_name = creation_common::chain_name::get(matches);
        let payment_amount = creation_common::payment_amount::get(matches)
            .ok_or(CliError::InvalidArgument {
                context: "Make Transaction",
                error: "Missing payment amount".to_string()
            })?;


        let session_str_params = creation_common::session_str_params(matches);

        let maybe_output_path = creation_common::output::get(matches).unwrap_or_default();
        let session_account = creation_common::session_account::get(matches).unwrap_or_default();

        let force = common::force::get(matches);

        casper_client::cli::make_transaction(
            maybe_output_path,
            TransactionStrParams {
                secret_key,
                timestamp,
                ttl,
                chain_name,
                initiator_addr: &session_account,
            },
            session_str_params,
            payment_amount,
            force,
        )
            .map(|_| {
                Success::Output(if maybe_output_path.is_empty() {
                    String::new()
                } else {
                    format!("Wrote the deploy to {}", maybe_output_path)
                })
            })
    }
}
