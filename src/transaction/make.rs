use async_trait::async_trait;
use clap::{ArgMatches, Command};

use casper_client::cli::CliError;

use super::creation_common::{
    add_bid, delegate, invocable_entity, invocable_entity_alias, package, package_alias,
    redelegate, session, transfer, undelegate, withdraw_bid, DisplayOrder,
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
        Command::new(Self::NAME)
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
            .arg(common::force::arg(DisplayOrder::Force as usize, true))
            .display_order(display_order)
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        let force = common::force::get(matches);

        if let Some((subcommand, matches)) = matches.subcommand() {
            let (transaction_builder_params, transaction_str_params) = match subcommand {
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
            let output_path = transaction_str_params.output_path;

            casper_client::cli::make_transaction(
                transaction_builder_params,
                transaction_str_params,
                force,
            )
            .map(|_| {
                Success::Output(if output_path.is_empty() {
                    String::new()
                } else {
                    format!("Wrote the deploy to {}", output_path)
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
