use async_trait::async_trait;
use clap::{ArgMatches, Command};

use casper_client::cli::CliError;

use super::creation_common::{
    add_bid, delegate, invocable_entity, invocable_entity_alias, package, package_alias,
    redelegate, session, transfer, undelegate, withdraw_bid, DisplayOrder,
};

use crate::{command::ClientCommand, common, Success};

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
            .arg(common::verbose::arg(DisplayOrder::Verbose as usize))
            .arg(common::rpc_id::arg(DisplayOrder::RpcId as usize))
            .arg(common::node_address::arg(
                DisplayOrder::NodeAddress as usize,
            ))
            .display_order(display_order)
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        let rpc_id = common::rpc_id::get(matches);
        let node_address = common::node_address::get(matches);
        let verbosity_level = common::verbose::get(matches);
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
