use async_trait::async_trait;
use clap::{ArgMatches, Command};

use casper_client::cli::CliError;

use super::creation_common::{
    add_bid, delegate, invocable_entity, invocable_entity_alias, package, package_alias,
    redelegate, session, transfer, undelegate, withdraw_bid,
};

use crate::{
    command::ClientCommand, transaction::creation_common::parse_rpc_args_and_run, Success,
};

pub struct PutTransaction;
const ALIAS: &str = "put-txn";
#[async_trait]
impl ClientCommand for PutTransaction {
    const NAME: &'static str = "put-transaction";

    const ABOUT: &'static str =
        "Create a transaction and output it to a file or stdout. As a file, the transaction can subsequently \
        be signed by other parties using the 'sign-transaction' subcommand and then sent to the network \
        for execution using the 'send-transaction' subcommand";

    fn build(display_order: usize) -> Command {
        Command::new(Self::NAME)
            .about(Self::ABOUT)
            .alias(ALIAS)
            .subcommand_required(true)
            .subcommand(withdraw_bid::put_transaction_build())
            .subcommand(add_bid::put_transaction_build())
            .subcommand(delegate::put_transaction_build())
            .subcommand(undelegate::put_transaction_build())
            .subcommand(redelegate::put_transaction_build())
            .subcommand(invocable_entity::put_transaction_build())
            .subcommand(invocable_entity_alias::put_transaction_build())
            .subcommand(package::put_transaction_build())
            .subcommand(package_alias::put_transaction_build())
            .subcommand(session::put_transaction_build())
            .subcommand(transfer::put_transaction_build())
            .display_order(display_order)
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        if let Some((subcommand, matches)) = matches.subcommand() {
            let (
                transaction_builder_params,
                transaction_str_params,
                node_address,
                rpc_id,
                verbosity_level,
            ) = match subcommand {
                add_bid::NAME => parse_rpc_args_and_run(matches, add_bid::run)?,
                withdraw_bid::NAME => parse_rpc_args_and_run(matches, withdraw_bid::run)?,
                delegate::NAME => parse_rpc_args_and_run(matches, delegate::run)?,
                undelegate::NAME => parse_rpc_args_and_run(matches, undelegate::run)?,
                redelegate::NAME => parse_rpc_args_and_run(matches, redelegate::run)?,
                invocable_entity::NAME => parse_rpc_args_and_run(matches, invocable_entity::run)?,
                invocable_entity_alias::NAME => {
                    parse_rpc_args_and_run(matches, invocable_entity_alias::run)?
                }
                package::NAME => parse_rpc_args_and_run(matches, package::run)?,
                package_alias::NAME => parse_rpc_args_and_run(matches, package_alias::run)?,
                session::NAME => parse_rpc_args_and_run(matches, session::run)?,
                transfer::NAME => parse_rpc_args_and_run(matches, transfer::run)?,
                _ => {
                    return Err(CliError::InvalidArgument {
                        context: "Make Transaction",
                        error: "failure to provide recognized subcommand".to_string(),
                    })
                }
            };
            casper_client::cli::put_transaction(
                rpc_id,
                node_address,
                verbosity_level,
                transaction_builder_params,
                transaction_str_params,
            )
            .await
            .map(Success::from)
        } else {
            return Err(CliError::InvalidArgument {
                context: "Put Transaction",
                error: "Failure to supply subcommand".to_string(),
            });
        }
    }
}
