mod account_address;
mod block;
mod command;
mod common;
mod deploy;
mod generate_completion;
mod get_account;
mod get_auction_info;
mod get_balance;
mod get_chainspec;
mod get_dictionary_item;
mod get_entity;
mod get_era_info;
mod get_era_summary;
mod get_node_status;
mod get_peers;
mod get_reward;
mod get_state_root_hash;
mod get_validator_changes;
mod keygen;
mod list_rpcs;
mod query_balance;
mod query_balance_details;
mod query_global_state;
mod transaction;
mod verify_contract;

use std::process;

use clap::{crate_version, Command};
use get_balance::GetBalance;
use get_entity::GetEntity;
use get_reward::GetReward;
use once_cell::sync::Lazy;

use casper_client::{cli, rpcs::results::GetChainspecResult, SuccessResponse};
use query_balance_details::QueryBalanceDetails;

use crate::transaction::GetTransaction;
use account_address::AccountAddress;
use block::{GetBlock, GetBlockTransfers};
use command::{ClientCommand, Success};
use deploy::{
    GetDeploy, ListDeploys, MakeDeploy, MakeTransfer, PutDeploy, SendDeploy, SignDeploy, Transfer,
};
use generate_completion::GenerateCompletion;
use get_account::GetAccount;
use get_auction_info::GetAuctionInfo;
use get_chainspec::GetChainspec;
use get_dictionary_item::GetDictionaryItem;
use get_era_info::GetEraInfo;
use get_era_summary::GetEraSummary;
use get_node_status::GetNodeStatus;
use get_peers::GetPeers;
use get_state_root_hash::GetStateRootHash;
use get_validator_changes::GetValidatorChanges;
use keygen::Keygen;
use list_rpcs::ListRpcs;
use query_balance::QueryBalance;
use query_global_state::QueryGlobalState;
use transaction::{MakeTransaction, PutTransaction, SendTransaction, SignTransaction};
use verify_contract::VerifyContract;

const APP_NAME: &str = "Casper client";

static VERSION: Lazy<String> =
    Lazy::new(
        || match option_env!("GIT_SHA_SHORT").map(|sha| sha.to_lowercase()) {
            None => crate_version!().to_string(),
            Some(git_sha_short) => {
                if git_sha_short.to_lowercase() == "unknown" {
                    crate_version!().to_string()
                } else {
                    format!("{}-{}", crate_version!(), git_sha_short)
                }
            }
        },
    );

/// This struct defines the order in which the subcommands are shown in the app's help message.
enum DisplayOrder {
    PutDeploy,
    MakeDeploy,
    PutTransaction,
    MakeTransaction,
    SignDeploy,
    SignTransaction,
    SendDeploy,
    SendTransaction,
    Transfer,
    MakeTransfer,
    GetDeploy,
    GetTransaction,
    GetBalance,
    GetBlock,
    GetBlockTransfers,
    ListDeploys,
    GetStateRootHash,
    GetEraSummary,
    GetEraInfo,
    QueryGlobalState,
    QueryBalance,
    QueryBalanceDetails,
    GetDictionaryItem,
    GetAccount,
    GetEntity,
    GetReward,
    GetAuctionInfo,
    GetValidatorChanges,
    GetPeers,
    GetNodeStatus,
    GetChainspec,
    ListRpcs,
    Keygen,
    AccountAddress,
    GenerateCompletion,
    VerifyContract,
}

fn cli() -> Command {
    Command::new(APP_NAME)
        .version(VERSION.as_str())
        .about("A client for interacting with the Casper network")
        .subcommand(PutDeploy::build(DisplayOrder::PutDeploy as usize))
        .subcommand(MakeDeploy::build(DisplayOrder::MakeDeploy as usize))
        .subcommand(PutTransaction::build(DisplayOrder::PutTransaction as usize))
        .subcommand(MakeTransaction::build(
            DisplayOrder::MakeTransaction as usize,
        ))
        .subcommand(SignDeploy::build(DisplayOrder::SignDeploy as usize))
        .subcommand(SignTransaction::build(
            DisplayOrder::SignTransaction as usize,
        ))
        .subcommand(SendDeploy::build(DisplayOrder::SendDeploy as usize))
        .subcommand(SendTransaction::build(
            DisplayOrder::SendTransaction as usize,
        ))
        .subcommand(Transfer::build(DisplayOrder::Transfer as usize))
        .subcommand(MakeTransfer::build(DisplayOrder::MakeTransfer as usize))
        .subcommand(GetBalance::build(DisplayOrder::GetBalance as usize).hide(true))
        .subcommand(GetDeploy::build(DisplayOrder::GetDeploy as usize))
        .subcommand(GetTransaction::build(DisplayOrder::GetTransaction as usize))
        .subcommand(GetBlock::build(DisplayOrder::GetBlock as usize))
        .subcommand(GetBlockTransfers::build(
            DisplayOrder::GetBlockTransfers as usize,
        ))
        .subcommand(ListDeploys::build(DisplayOrder::ListDeploys as usize))
        .subcommand(GetStateRootHash::build(
            DisplayOrder::GetStateRootHash as usize,
        ))
        .subcommand(GetEraSummary::build(DisplayOrder::GetEraSummary as usize))
        .subcommand(GetEraInfo::build(DisplayOrder::GetEraInfo as usize))
        .subcommand(QueryGlobalState::build(
            DisplayOrder::QueryGlobalState as usize,
        ))
        .subcommand(QueryBalance::build(DisplayOrder::QueryBalance as usize))
        .subcommand(QueryBalanceDetails::build(
            DisplayOrder::QueryBalanceDetails as usize,
        ))
        .subcommand(GetDictionaryItem::build(
            DisplayOrder::GetDictionaryItem as usize,
        ))
        .subcommand(GetAccount::build(DisplayOrder::GetAccount as usize))
        .subcommand(GetEntity::build(DisplayOrder::GetEntity as usize))
        .subcommand(GetReward::build(DisplayOrder::GetReward as usize))
        .subcommand(GetAuctionInfo::build(DisplayOrder::GetAuctionInfo as usize))
        .subcommand(GetValidatorChanges::build(
            DisplayOrder::GetValidatorChanges as usize,
        ))
        .subcommand(GetPeers::build(DisplayOrder::GetPeers as usize))
        .subcommand(GetNodeStatus::build(DisplayOrder::GetNodeStatus as usize))
        .subcommand(GetChainspec::build(DisplayOrder::GetChainspec as usize))
        .subcommand(ListRpcs::build(DisplayOrder::ListRpcs as usize))
        .subcommand(Keygen::build(DisplayOrder::Keygen as usize))
        .subcommand(AccountAddress::build(DisplayOrder::AccountAddress as usize))
        .subcommand(GenerateCompletion::build(
            DisplayOrder::GenerateCompletion as usize,
        ))
        .subcommand(VerifyContract::build(DisplayOrder::VerifyContract as usize))
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let arg_matches = cli().get_matches();
    let (subcommand_name, matches) = arg_matches.subcommand().unwrap_or_else(|| {
        let _ = cli().print_long_help();
        println!();
        process::exit(1);
    });

    let result = match subcommand_name {
        PutDeploy::NAME => PutDeploy::run(matches).await,
        MakeDeploy::NAME => MakeDeploy::run(matches).await,
        PutTransaction::NAME => PutTransaction::run(matches).await,
        MakeTransaction::NAME => MakeTransaction::run(matches).await,
        SignDeploy::NAME => SignDeploy::run(matches).await,
        SignTransaction::NAME => SignTransaction::run(matches).await,
        SendDeploy::NAME => SendDeploy::run(matches).await,
        SendTransaction::NAME => SendTransaction::run(matches).await,
        Transfer::NAME => Transfer::run(matches).await,
        MakeTransfer::NAME => MakeTransfer::run(matches).await,
        GetDeploy::NAME => GetDeploy::run(matches).await,
        GetTransaction::NAME => GetTransaction::run(matches).await,
        GetBalance::NAME => GetBalance::run(matches).await,
        GetBlock::NAME => GetBlock::run(matches).await,
        GetBlockTransfers::NAME => GetBlockTransfers::run(matches).await,
        ListDeploys::NAME => ListDeploys::run(matches).await,
        GetStateRootHash::NAME => GetStateRootHash::run(matches).await,
        GetEraSummary::NAME => GetEraSummary::run(matches).await,
        GetEraInfo::NAME => GetEraInfo::run(matches).await,
        QueryGlobalState::NAME => QueryGlobalState::run(matches).await,
        QueryBalance::NAME => QueryBalance::run(matches).await,
        QueryBalanceDetails::NAME => QueryBalanceDetails::run(matches).await,
        GetDictionaryItem::NAME => GetDictionaryItem::run(matches).await,
        GetAccount::NAME => GetAccount::run(matches).await,
        GetEntity::NAME => GetEntity::run(matches).await,
        GetReward::NAME => GetReward::run(matches).await,
        GetAuctionInfo::NAME => GetAuctionInfo::run(matches).await,
        GetValidatorChanges::NAME => GetValidatorChanges::run(matches).await,
        GetPeers::NAME => GetPeers::run(matches).await,
        GetNodeStatus::NAME => GetNodeStatus::run(matches).await,
        GetChainspec::NAME => GetChainspec::run(matches).await,
        ListRpcs::NAME => ListRpcs::run(matches).await,
        Keygen::NAME => Keygen::run(matches).await,
        AccountAddress::NAME => AccountAddress::run(matches).await,
        GenerateCompletion::NAME => GenerateCompletion::run(matches).await,
        VerifyContract::NAME => VerifyContract::run(matches).await,
        _ => {
            let _ = cli().print_long_help();
            println!();
            process::exit(1);
        }
    };

    let mut verbosity_level = common::verbose::get(matches);
    if verbosity_level == 0 {
        verbosity_level += 1;
    }

    match result {
        Ok(Success::Response(response)) => {
            cli::json_pretty_print(&response, verbosity_level).expect("should print");
            if verbosity_level > 1 && subcommand_name == GetChainspec::NAME {
                let success_response: SuccessResponse<GetChainspecResult> =
                    serde_json::from_value(response).expect("should be a chainspec result");
                println!("{}", success_response.result.chainspec_bytes);
            }
        }
        Ok(Success::Output(output)) => println!("{}", output),
        Err(error) => {
            println!("{}", error);
            process::exit(1);
        }
    }
}
