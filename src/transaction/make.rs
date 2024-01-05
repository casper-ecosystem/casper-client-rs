use async_trait::async_trait;
use casper_types::TransactionV1Builder;
use clap::{ArgMatches, Command};

use casper_client::cli::{CliError, TransactionStrParams};

use super::creation_common::{
    self, add_bid, arg_simple, args_json, delegate, invocable_entity, invocable_entity_alias,
    package, package_alias, redelegate, session, transfer, undelegate, withdraw_bid,
};

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
            .subcommand_required(true)
            .subcommand(withdraw_bid::build())
            .subcommand(add_bid::build())
            .subcommand(delegate::build())
            .subcommand(undelegate::build())
            .subcommand(redelegate::build())
            .subcommand(invocable_entity::build())
            .subcommand(invocable_entity_alias::build())
            .subcommand(package::build())
            .subcommand(package_alias::build())
            .subcommand(session::build())
            .subcommand(transfer::build())
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
        let payment_amount =
            creation_common::payment_amount::get(matches).ok_or(CliError::InvalidArgument {
                context: "Make Transaction",
                error: "Missing payment amount".to_string(),
            })?;

        let maybe_pricing_mode = creation_common::pricing_mode::get(matches);

        let session_path = creation_common::transaction_path::get(matches);
        let session_entry_point = creation_common::session_entry_point::get(matches);
        let session_args_simple = arg_simple::session::get(matches);
        let session_args_json = args_json::session::get(matches);

        let maybe_output_path = creation_common::output::get(matches).unwrap_or_default();
        let initiator_addr = creation_common::initiator_address::get(matches).unwrap_or_default();

        let force = common::force::get(matches);

        if let Some((subcommand, matches)) = matches.subcommand() {
            let transaction_builder: TransactionV1Builder = match subcommand {
                add_bid::NAME => add_bid::run(matches)?,
                withdraw_bid::NAME => withdraw_bid::run(matches)?,
                delegate::NAME => delegate::run(matches)?,
                undelegate::NAME => undelegate::run(matches)?,
                redelegate::NAME => redelegate::run(matches)?,
                invocable_entity::NAME => invocable_entity::run(matches)?,
                invocable_entity_alias::NAME => invocable_entity_alias::run(matches)?,
                package::NAME => package::run(matches)?,
                package_alias::NAME => package_alias::run(matches)?,
                session::NAME => session::run(matches)?,
                transfer::NAME => transfer::run(matches)?,
                _ => {
                    return Err(CliError::InvalidArgument {
                        context: "Make Transaction",
                        error: "failure to provide recognized subcommand".to_string(),
                    })
                }
            };

            casper_client::cli::make_transaction(
                maybe_output_path,
                transaction_builder,
                TransactionStrParams {
                    secret_key,
                    timestamp,
                    ttl,
                    chain_name,
                    initiator_addr,
                    session_path,
                    session_entry_point,
                    session_args_simple,
                    session_args_json,
                    maybe_pricing_mode,
                },
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
        } else {
            return Err(CliError::InvalidArgument {
                context: "Make Transaction",
                error: "Failure to supply subcommand".to_string(),
            });
        }
    }
}
