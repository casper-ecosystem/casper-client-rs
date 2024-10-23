//! # Casper client library
//!
//! The crate provides functions for interacting with a Casper network.
//!
//! Most of the functions involve sending a JSON-RPC request to a specified node on the chosen
//! network, and providing the RPC response.
//!
//! # Common Parameters
//!
//! Many of the functions have similar parameters.  Descriptions for these common ones follow:
//!
//! * <code>rpc_id: <a href="enum.JsonRpcId.html">JsonRpcId</a></code> - The JSON-RPC identifier,
//!   applied to the request and returned in the response.
//! * <code>node_address: &<a href="https://doc.rust-lang.org/std/primitive.str.html">str</a></code> -
//!   The hostname or IP and port of the server, e.g. `http://127.0.0.1:7777`.
//! * <code>verbosity: <a href="enum.Verbosity.html">Verbosity</a></code> - When `Low`, nothing is
//!   printed to stdout.  For `Medium`, the request and response are printed to `stdout` with long
//!   string fields (e.g. hex-formatted raw Wasm bytes) shortened to a string indicating the char
//!   count of the field.  `High` verbosity is the same as `Medium` except without abbreviation of
//!   long fields.
//! * <code>maybe_block_identifier: <a href="https://doc.rust-lang.org/core/option/enum.Option.html">Option</a><<a href="rpcs/common/enum.BlockIdentifier.html">BlockIdentifier</a>></code> -
//!   The identifier of the [`Block`] to use, either block height or block hash.  If `None`, the
//!   latest `Block` known on the server will be used.

#![doc(
    html_root_url = "https://docs.rs/casper-client/2.0.0",
    html_favicon_url = "https://raw.githubusercontent.com/CasperLabs/casper-node/master/images/CasperLabs_Logo_Favicon_RGB_50px.png",
    html_logo_url = "https://raw.githubusercontent.com/CasperLabs/casper-node/master/images/CasperLabs_Logo_Symbol_RGB.png",
    test(attr(forbid(warnings)))
)]
#![warn(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_qualifications
)]

pub mod cli;
mod error;
mod json_rpc;
#[cfg(feature = "std-fs-io")]
pub mod keygen;
#[cfg(any(feature = "std-fs-io", test))]
mod output_kind;
pub mod rpcs;
pub mod types;
mod validation;
mod verbosity;
mod verification;
mod verification_types;

#[cfg(any(feature = "std-fs-io", test))]
use std::{
    env::current_dir,
    fs,
    io::{Cursor, Read, Write},
    path::Path,
};

#[cfg(feature = "std-fs-io")]
use serde::Serialize;

#[cfg(doc)]
use casper_types::{account::Account, Block, StoredValue, Transfer};
use casper_types::{
    Deploy, DeployHash, Digest, Key, PublicKey, Transaction, TransactionHash, URef,
};
#[cfg(any(feature = "std-fs-io", test))]
use casper_types::{SecretKey, TransactionV1};

#[cfg(any(feature = "std-fs-io", test))]
use base64::{engine::general_purpose::STANDARD, Engine};
pub use error::Error;
use json_rpc::JsonRpcCall;
pub use json_rpc::{JsonRpcId, SuccessResponse};
#[cfg(any(feature = "std-fs-io", test))]
pub use output_kind::OutputKind;
use rpcs::{
    common::{BlockIdentifier, GlobalStateIdentifier},
    results::{
        GetAccountResult, GetAddressableEntityResult, GetAuctionInfoResult, GetBalanceResult,
        GetBlockResult, GetBlockTransfersResult, GetChainspecResult, GetDeployResult,
        GetDictionaryItemResult, GetEraInfoResult, GetEraSummaryResult, GetNodeStatusResult,
        GetPeersResult, GetRewardResult, GetStateRootHashResult, GetTransactionResult,
        GetValidatorChangesResult, ListRpcsResult, PutDeployResult, PutTransactionResult,
        QueryBalanceDetailsResult, QueryBalanceResult, QueryGlobalStateResult,
        SpeculativeExecResult, SpeculativeExecTxnResult,
    },
    v2_0_0::{
        get_account::{AccountIdentifier, GetAccountParams, GET_ACCOUNT_METHOD},
        get_auction_info::{GetAuctionInfoParams, GET_AUCTION_INFO_METHOD},
        get_balance::{GetBalanceParams, GET_BALANCE_METHOD},
        get_block::{GetBlockParams, GET_BLOCK_METHOD},
        get_block_transfers::{GetBlockTransfersParams, GET_BLOCK_TRANSFERS_METHOD},
        get_chainspec::GET_CHAINSPEC_METHOD,
        get_deploy::{GetDeployParams, GET_DEPLOY_METHOD},
        get_dictionary_item::{GetDictionaryItemParams, GET_DICTIONARY_ITEM_METHOD},
        get_entity::{EntityIdentifier, GetAddressableEntityParams, GET_ENTITY_METHOD},
        get_era_info::{GetEraInfoParams, GET_ERA_INFO_METHOD},
        get_era_summary::{GetEraSummaryParams, GET_ERA_SUMMARY_METHOD},
        get_node_status::GET_NODE_STATUS_METHOD,
        get_peers::GET_PEERS_METHOD,
        get_reward::{GetRewardParams, GET_REWARD_METHOD},
        get_state_root_hash::{GetStateRootHashParams, GET_STATE_ROOT_HASH_METHOD},
        get_transaction::{GetTransactionParams, GET_TRANSACTION_METHOD},
        get_validator_changes::GET_VALIDATOR_CHANGES_METHOD,
        list_rpcs::LIST_RPCS_METHOD,
        put_deploy::{PutDeployParams, PUT_DEPLOY_METHOD},
        put_transaction::{PutTransactionParams, PUT_TRANSACTION_METHOD},
        query_balance::{PurseIdentifier, QueryBalanceParams, QUERY_BALANCE_METHOD},
        query_balance_details::{QueryBalanceDetailsParams, QUERY_BALANCE_DETAILS_METHOD},
        query_global_state::{QueryGlobalStateParams, QUERY_GLOBAL_STATE_METHOD},
        speculative_exec::{SpeculativeExecParams, SPECULATIVE_EXEC_METHOD},
        speculative_exec_transaction::{SpeculativeExecTxnParams, SPECULATIVE_EXEC_TXN_METHOD},
    },
    DictionaryItemIdentifier, EraIdentifier,
};
pub use validation::ValidateResponseError;
pub use verbosity::Verbosity;
pub use verification::{build_archive, send_verification_request};
#[cfg(any(feature = "std-fs-io", test))]
use verification_types::VerificationDetails;

/// The maximum permissible size in bytes of a Deploy when serialized via `ToBytes`.
///
/// Note: this should be kept in sync with the value of `[deploys.max_deploy_size]` in the
/// production chainspec.
pub const MAX_SERIALIZED_SIZE_OF_DEPLOY: u32 = 1_024 * 1_024;

/// Puts a [`Deploy`] to the network for execution.
///
/// Sends a JSON-RPC `account_put_deploy` request to the specified node.
///
/// For details of the parameters, see [the module docs](crate#common-parameters).
pub async fn put_deploy(
    rpc_id: JsonRpcId,
    node_address: &str,
    verbosity: Verbosity,
    deploy: Deploy,
) -> Result<SuccessResponse<PutDeployResult>, Error> {
    JsonRpcCall::new(rpc_id, node_address, verbosity)
        .send_request(PUT_DEPLOY_METHOD, Some(PutDeployParams::new(deploy)))
        .await
}

/// Puts a [`Transaction`] to the network for execution
///
/// Sends a JSON-RPC `account_put_transaction` request to the specified node.
///
/// For details of the parameters, see [the module docs](crate#common-parameters).
pub async fn put_transaction(
    rpc_id: JsonRpcId,
    node_address: &str,
    verbosity: Verbosity,
    transaction: Transaction,
) -> Result<SuccessResponse<PutTransactionResult>, Error> {
    JsonRpcCall::new(rpc_id, node_address, verbosity)
        .send_request(
            PUT_TRANSACTION_METHOD,
            Some(PutTransactionParams::new(transaction)),
        )
        .await
}

/// Puts a [`Deploy`] to a single node for speculative execution on that node only.
///
/// Sends a JSON-RPC `speculative_exec` request to the specified node.
///
/// For details of the parameters, see [the module docs](crate#common-parameters).
pub async fn speculative_exec(
    rpc_id: JsonRpcId,
    node_address: &str,
    verbosity: Verbosity,
    deploy: Deploy,
) -> Result<SuccessResponse<SpeculativeExecResult>, Error> {
    JsonRpcCall::new(rpc_id, node_address, verbosity)
        .send_request(
            SPECULATIVE_EXEC_METHOD,
            Some(SpeculativeExecParams::new(deploy)),
        )
        .await
}

/// Puts a [`Transaction`] to a single node for speculative execution on that node only.
///
/// Sends a JSON-RPC speculative_exec request to the specified node.
///
/// For details of the parameters, see [the module docs](crate#common-parameters).
pub async fn speculative_exec_txn(
    rpc_id: JsonRpcId,
    node_address: &str,
    verbosity: Verbosity,
    transaction: Transaction,
) -> Result<SuccessResponse<SpeculativeExecTxnResult>, Error> {
    JsonRpcCall::new(rpc_id, node_address, verbosity)
        .send_request(
            SPECULATIVE_EXEC_TXN_METHOD,
            Some(SpeculativeExecTxnParams::new(transaction)),
        )
        .await
}

/// Outputs a [`Deploy`] to a file or stdout.
///
/// As a file, the `Deploy` can subsequently be signed by other parties using [`sign_deploy_file`]
/// and then read and sent to the network for execution using [`read_deploy_file`] and
/// [`put_deploy`] respectively.
///
/// `output` specifies the output file and corresponding overwrite behaviour, or if
/// `OutputKind::Stdout`, causes the `Deploy` to be printed `stdout`.
#[cfg(any(feature = "std-fs-io", test))]
pub fn output_deploy(output: OutputKind, deploy: &Deploy) -> Result<(), Error> {
    write_deploy(deploy, output.get()?)?;
    output.commit()
}

/// Outputs a [`Transaction`] to a file or stdout.
///
/// As a file, the `Transaction` can subsequently be signed by other parties using [`sign_transaction_file`]
/// and then read and sent to the network for execution using [`read_transaction_file`] and
/// [`put_transaction`] respectively.
///
/// `output` specifies the output file and corresponding overwrite behaviour, or if
/// `OutputKind::Stdout`, causes the `Transaction` to be printed `stdout`.
#[cfg(any(feature = "std-fs-io", test))]
pub fn output_transaction(output: OutputKind, transaction: &TransactionV1) -> Result<(), Error> {
    write_transaction(transaction, output.get()?)?;
    output.commit()
}

/// Reads a previously-saved [`Deploy`] from a file.
#[cfg(any(feature = "std-fs-io", test))]
pub fn read_deploy_file<P: AsRef<Path>>(deploy_path: P) -> Result<Deploy, Error> {
    let input = fs::read(deploy_path.as_ref()).map_err(|error| Error::IoError {
        context: format!(
            "unable to read deploy file at '{}'",
            deploy_path.as_ref().display()
        ),
        error,
    })?;
    read_deploy(Cursor::new(input))
}

/// Reads a previously-saved [`Transaction`] from a file.
#[cfg(any(feature = "std-fs-io", test))]
pub fn read_transaction_file<P: AsRef<Path>>(transaction_path: P) -> Result<TransactionV1, Error> {
    let input = fs::read(transaction_path.as_ref()).map_err(|error| Error::IoError {
        context: format!(
            "unable to read transaction file at '{}'",
            transaction_path.as_ref().display()
        ),
        error,
    })?;
    read_transaction(Cursor::new(input))
}

/// Reads a previously-saved [`Deploy`] from a file, cryptographically signs it, and outputs it
/// to a file or stdout.
///
/// `output` specifies the output file and corresponding overwrite behaviour, or if
/// `OutputKind::Stdout`, causes the `Deploy` to be printed `stdout`.
///
/// The same path can be specified for input and output, and if the operation fails, the original
/// input file will be left unmodified.
#[cfg(any(feature = "std-fs-io", test))]
pub fn sign_deploy_file<P: AsRef<Path>>(
    input_path: P,
    secret_key: &SecretKey,
    output: OutputKind,
) -> Result<(), Error> {
    let mut deploy = read_deploy_file(input_path)?;

    deploy.sign(secret_key);
    deploy.is_valid_size(MAX_SERIALIZED_SIZE_OF_DEPLOY)?;

    write_deploy(&deploy, output.get()?)?;
    output.commit()
}

/// Reads a previously-saved [`TransactionV1`] from a file, cryptographically signs it, and outputs it to a file or stdout.
///
/// `output` specifies the output file and corresponding overwrite behaviour, or if OutputKind::Stdout,
/// causes the `Transaction` to be printed `stdout`.
#[cfg(any(feature = "std-fs-io", test))]
pub fn sign_transaction_file<P: AsRef<Path>>(
    input_path: P,
    secret_key: &SecretKey,
    output: OutputKind,
) -> Result<(), Error> {
    let mut transaction = read_transaction_file(input_path)?;

    transaction.sign(secret_key);

    write_transaction(&transaction, output.get()?)?;
    output.commit()
}

/// Retrieves a [`Deploy`] and its metadata (i.e. execution results) from the network.
///
/// Sends a JSON-RPC `info_get_deploy` request to the specified node.
///
/// `finalized_approvals` defines whether to return the `Deploy` with its approvals as finalized by
/// consensus of the validators on the network, or as originally received by the specified node.
///
/// For details of the other parameters, see [the module docs](crate#common-parameters).
pub async fn get_deploy(
    rpc_id: JsonRpcId,
    node_address: &str,
    verbosity: Verbosity,
    deploy_hash: DeployHash,
    finalized_approvals: bool,
) -> Result<SuccessResponse<GetDeployResult>, Error> {
    JsonRpcCall::new(rpc_id, node_address, verbosity)
        .send_request(
            GET_DEPLOY_METHOD,
            Some(GetDeployParams::new(deploy_hash, finalized_approvals)),
        )
        .await
}

/// Retrieves a [`Transaction`] and its metadata (i.e. execution results) from the network.
///
/// Sends a JSON-RPC `info_get_transaction` request to the specified node.
///
/// `finalized_approvals` defines whether to return the `Transaction` with its approvals as finalized by
/// consensus of the validators on the network, or as originally received by the specified node.
///
/// For details of the other parameters, see [the module docs](crate#common-parameters).
pub async fn get_transaction(
    rpc_id: JsonRpcId,
    node_address: &str,
    verbosity: Verbosity,
    transaction_hash: TransactionHash,
    finalized_approvals: bool,
) -> Result<SuccessResponse<GetTransactionResult>, Error> {
    JsonRpcCall::new(rpc_id, node_address, verbosity)
        .send_request(
            GET_TRANSACTION_METHOD,
            Some(GetTransactionParams::new(
                transaction_hash,
                finalized_approvals,
            )),
        )
        .await
}

/// Retrieves a [`Block`] from the network.
///
/// Sends a JSON-RPC `chain_get_block` request to the specified node.
///
/// For details of the parameters, see [the module docs](crate#common-parameters).
pub async fn get_block(
    rpc_id: JsonRpcId,
    node_address: &str,
    verbosity: Verbosity,
    maybe_block_identifier: Option<BlockIdentifier>,
) -> Result<SuccessResponse<GetBlockResult>, Error> {
    let params = maybe_block_identifier.map(GetBlockParams::new);
    let success_response = JsonRpcCall::new(rpc_id, node_address, verbosity)
        .send_request(GET_BLOCK_METHOD, params)
        .await?;
    validation::validate_get_block_result(maybe_block_identifier, &success_response.result)?;
    Ok(success_response)
}

/// Retrieves all [`Transfer`] items for a given [`Block`].
///
/// Sends a JSON-RPC `chain_get_block_transfers` request to the specified node.
///
/// For details of the parameters, see [the module docs](crate#common-parameters).
pub async fn get_block_transfers(
    rpc_id: JsonRpcId,
    node_address: &str,
    verbosity: Verbosity,
    maybe_block_identifier: Option<BlockIdentifier>,
) -> Result<SuccessResponse<GetBlockTransfersResult>, Error> {
    let params = maybe_block_identifier.map(GetBlockTransfersParams::new);
    JsonRpcCall::new(rpc_id, node_address, verbosity)
        .send_request(GET_BLOCK_TRANSFERS_METHOD, params)
        .await
}

/// Retrieves a state root hash at a given [`Block`].
///
/// Sends a JSON-RPC `chain_get_state_root_hash` request to the specified node.
///
/// For details of the parameters, see [the module docs](crate#common-parameters).
pub async fn get_state_root_hash(
    rpc_id: JsonRpcId,
    node_address: &str,
    verbosity: Verbosity,
    maybe_block_identifier: Option<BlockIdentifier>,
) -> Result<SuccessResponse<GetStateRootHashResult>, Error> {
    let params = maybe_block_identifier.map(GetStateRootHashParams::new);
    JsonRpcCall::new(rpc_id, node_address, verbosity)
        .send_request(GET_STATE_ROOT_HASH_METHOD, params)
        .await
}

/// Retrieves era information from the network at a given [`Block`].
///
/// Sends a JSON-RPC `chain_get_era_summary` request to the specified node.
///
/// For details of the parameters, see [the module docs](crate#common-parameters).
pub async fn get_era_summary(
    rpc_id: JsonRpcId,
    node_address: &str,
    verbosity: Verbosity,
    maybe_block_identifier: Option<BlockIdentifier>,
) -> Result<SuccessResponse<GetEraSummaryResult>, Error> {
    let params = maybe_block_identifier.map(GetEraSummaryParams::new);
    JsonRpcCall::new(rpc_id, node_address, verbosity)
        .send_request(GET_ERA_SUMMARY_METHOD, params)
        .await
}

/// Retrieves a [`StoredValue`] from global state at a given [`Block`] or state root hash.
///
/// Sends a JSON-RPC `query_global_state` request to the specified node.
///
/// `key` specifies the key under which the value is stored in global state.
///
/// `path` defines the further path (if any) from `key` to navigate to during the query.  This is
/// only applicable in the case where the value under `key` is an account or contract.  In this
/// case, the first `path` element represents a name in the account/contract's named keys.  If that
/// second `Key` also points to an account or contract, then a second path element can be added to
/// continue the query into that account/contract's named keys.  This can continue up to the
/// server's configured maximum query depth (5 by default).
///
/// For details of the other parameters, see [the module docs](crate#common-parameters).
pub async fn query_global_state(
    rpc_id: JsonRpcId,
    node_address: &str,
    verbosity: Verbosity,
    global_state_identifier: GlobalStateIdentifier,
    key: Key,
    path: Vec<String>,
) -> Result<SuccessResponse<QueryGlobalStateResult>, Error> {
    let params = QueryGlobalStateParams::new(global_state_identifier, key, path);
    JsonRpcCall::new(rpc_id, node_address, verbosity)
        .send_request(QUERY_GLOBAL_STATE_METHOD, Some(params))
        .await
}

/// Retrieves a purse's balance from global state at a given [`Block`] or state root hash.
///
/// Sends a JSON-RPC `query_balance` request to the specified node.
///
/// For details of the parameters, see [the module docs](crate#common-parameters).
pub async fn query_balance(
    rpc_id: JsonRpcId,
    node_address: &str,
    verbosity: Verbosity,
    maybe_global_state_identifier: Option<GlobalStateIdentifier>,
    purse_identifier: PurseIdentifier,
) -> Result<SuccessResponse<QueryBalanceResult>, Error> {
    let params = QueryBalanceParams::new(maybe_global_state_identifier, purse_identifier);
    JsonRpcCall::new(rpc_id, node_address, verbosity)
        .send_request(QUERY_BALANCE_METHOD, Some(params))
        .await
}

/// Retrieves a purse's balance from global state at a given [`Block`] or state root hash.
///
/// Sends a JSON-RPC `query_balance_details` request to the specified node.
///
/// For details of the parameters, see [the module docs](crate#common-parameters).
pub async fn query_balance_details(
    rpc_id: JsonRpcId,
    node_address: &str,
    verbosity: Verbosity,
    maybe_global_state_identifier: Option<GlobalStateIdentifier>,
    purse_identifier: PurseIdentifier,
) -> Result<SuccessResponse<QueryBalanceDetailsResult>, Error> {
    let params = QueryBalanceDetailsParams::new(maybe_global_state_identifier, purse_identifier);
    JsonRpcCall::new(rpc_id, node_address, verbosity)
        .send_request(QUERY_BALANCE_DETAILS_METHOD, Some(params))
        .await
}

/// Retrieves a [`StoredValue`] from a dictionary at a given state root hash.
///
/// Sends a JSON-RPC `state_get_dictionary_item` request to the specified node.
///
/// For details of the parameters, see [the module docs](crate#common-parameters).
pub async fn get_dictionary_item(
    rpc_id: JsonRpcId,
    node_address: &str,
    verbosity: Verbosity,
    state_root_hash: Digest,
    dictionary_item_identifier: DictionaryItemIdentifier,
) -> Result<SuccessResponse<GetDictionaryItemResult>, Error> {
    let params = GetDictionaryItemParams::new(state_root_hash, dictionary_item_identifier);
    JsonRpcCall::new(rpc_id, node_address, verbosity)
        .send_request(GET_DICTIONARY_ITEM_METHOD, Some(params))
        .await
}

/// Retrieves a purse's balance at a given state root hash.
///
/// Sends a JSON-RPC `state_get_balance` request to the specified node.
///
/// For details of the parameters, see [the module docs](crate#common-parameters).
pub async fn get_balance(
    rpc_id: JsonRpcId,
    node_address: &str,
    verbosity: Verbosity,
    state_root_hash: Digest,
    purse: URef,
) -> Result<SuccessResponse<GetBalanceResult>, Error> {
    let params = GetBalanceParams::new(state_root_hash, purse);
    JsonRpcCall::new(rpc_id, node_address, verbosity)
        .send_request(GET_BALANCE_METHOD, Some(params))
        .await
}

/// Retrieves an [`Account`] at a given [`Block`].
///
/// Sends a JSON-RPC `state_get_account_info` request to the specified node.
///
/// For details of the parameters, see [the module docs](crate#common-parameters).
pub async fn get_account(
    rpc_id: JsonRpcId,
    node_address: &str,
    verbosity: Verbosity,
    maybe_block_identifier: Option<BlockIdentifier>,
    account_identifier: AccountIdentifier,
) -> Result<SuccessResponse<GetAccountResult>, Error> {
    let params = GetAccountParams::new(account_identifier, maybe_block_identifier);
    JsonRpcCall::new(rpc_id, node_address, verbosity)
        .send_request(GET_ACCOUNT_METHOD, Some(params))
        .await
}

/// Retrieves an [`crate::rpcs::v2_0_0::get_entity::EntityOrAccount`] at a given [`Block`].
///
/// Sends a JSON-RPC `state_get_entity` request to the specified node.
///
/// For details of the parameters, see [the module docs](crate#common-parameters).
pub async fn get_entity(
    rpc_id: JsonRpcId,
    node_address: &str,
    verbosity: Verbosity,
    maybe_block_identifier: Option<BlockIdentifier>,
    entity_identifier: EntityIdentifier,
) -> Result<SuccessResponse<GetAddressableEntityResult>, Error> {
    let params = GetAddressableEntityParams::new(entity_identifier, maybe_block_identifier);
    JsonRpcCall::new(rpc_id, node_address, verbosity)
        .send_request(GET_ENTITY_METHOD, Some(params))
        .await
}

/// Retrieves a [`GetRewardResult`] at a given [`EraIdentifier`].
///
/// Sends a JSON-RPC `info_get_reward` request to the specified node.
///
/// For details of the parameters, see [the module docs](crate#common-parameters).
pub async fn get_reward(
    rpc_id: JsonRpcId,
    node_address: &str,
    verbosity: Verbosity,
    maybe_era_identifier: Option<EraIdentifier>,
    validator: PublicKey,
    delegator: Option<PublicKey>,
) -> Result<SuccessResponse<GetRewardResult>, Error> {
    let params = GetRewardParams::new(maybe_era_identifier, validator, delegator);
    JsonRpcCall::new(rpc_id, node_address, verbosity)
        .send_request(GET_REWARD_METHOD, Some(params))
        .await
}

/// Retrieves the bids and validators at a given [`Block`].
///
/// Sends a JSON-RPC `state_get_auction_info` request to the specified node.
///
/// For details of the parameters, see [the module docs](crate#common-parameters).
pub async fn get_auction_info(
    rpc_id: JsonRpcId,
    node_address: &str,
    verbosity: Verbosity,
    maybe_block_identifier: Option<BlockIdentifier>,
) -> Result<SuccessResponse<GetAuctionInfoResult>, Error> {
    let params = maybe_block_identifier.map(GetAuctionInfoParams::new);
    JsonRpcCall::new(rpc_id, node_address, verbosity)
        .send_request(GET_AUCTION_INFO_METHOD, params)
        .await
}

/// Retrieves the status changes of the active validators on the network.
///
/// Sends a JSON-RPC `info_get_validator_changes` request to the specified node.
///
/// For details of the parameters, see [the module docs](crate#common-parameters).
pub async fn get_validator_changes(
    rpc_id: JsonRpcId,
    node_address: &str,
    verbosity: Verbosity,
) -> Result<SuccessResponse<GetValidatorChangesResult>, Error> {
    JsonRpcCall::new(rpc_id, node_address, verbosity)
        .send_request::<(), _>(GET_VALIDATOR_CHANGES_METHOD, None)
        .await
}

/// Retrieves the IDs and addresses of the specified node's peers.
///
/// Sends a JSON-RPC `info_get_peers` request to the specified node.
///
/// For details of the parameters, see [the module docs](crate#common-parameters).
pub async fn get_peers(
    rpc_id: JsonRpcId,
    node_address: &str,
    verbosity: Verbosity,
) -> Result<SuccessResponse<GetPeersResult>, Error> {
    JsonRpcCall::new(rpc_id, node_address, verbosity)
        .send_request::<(), _>(GET_PEERS_METHOD, None)
        .await
}

/// Retrieves the status of the specified node.
///
/// Sends a JSON-RPC `info_get_status` request to the specified node.
///
/// For details of the parameters, see [the module docs](crate#common-parameters).
pub async fn get_node_status(
    rpc_id: JsonRpcId,
    node_address: &str,
    verbosity: Verbosity,
) -> Result<SuccessResponse<GetNodeStatusResult>, Error> {
    JsonRpcCall::new(rpc_id, node_address, verbosity)
        .send_request::<(), _>(GET_NODE_STATUS_METHOD, None)
        .await
}

/// Retrieves the Chainspec of the network.
///
/// Sends a JSON-RPC `info_get_chainspec` request to the specified node.
///
/// For details of the parameters, see [the module docs](crate#common-parameters).
pub async fn get_chainspec(
    rpc_id: JsonRpcId,
    node_address: &str,
    verbosity: Verbosity,
) -> Result<SuccessResponse<GetChainspecResult>, Error> {
    JsonRpcCall::new(rpc_id, node_address, verbosity)
        .send_request::<(), _>(GET_CHAINSPEC_METHOD, None)
        .await
}

/// Retrieves the interface description (the schema including examples in OpenRPC format) of the
/// JSON-RPC server's API.
///
/// Sends a JSON-RPC `rpc.discover` request to the specified node.
///
/// For details of the parameters, see [the module docs](crate#common-parameters).
pub async fn list_rpcs(
    rpc_id: JsonRpcId,
    node_address: &str,
    verbosity: Verbosity,
) -> Result<SuccessResponse<ListRpcsResult>, Error> {
    JsonRpcCall::new(rpc_id, node_address, verbosity)
        .send_request::<(), _>(LIST_RPCS_METHOD, None)
        .await
}

/// JSON-encode and pretty-print the given value to stdout at the given verbosity level.
///
/// When `verbosity` is `Low`, nothing is printed.  For `Medium`, the value is printed with long
/// string fields shortened to a string indicating the character count of the field.  `High`
/// verbosity is the same as `Medium` except without abbreviation of long fields.
#[cfg(feature = "std-fs-io")]
pub(crate) fn json_pretty_print<T: ?Sized + Serialize>(
    value: &T,
    verbosity: Verbosity,
) -> Result<(), Error> {
    let output = match verbosity {
        Verbosity::Low => return Ok(()),
        Verbosity::Medium => casper_types::json_pretty_print(value),
        Verbosity::High => serde_json::to_string_pretty(value),
    }
    .map_err(|error| Error::FailedToEncodeToJson {
        context: "in json_pretty_print",
        error,
    })?;
    println!("{}", output);
    Ok(())
}

#[cfg(any(feature = "std-fs-io", test))]
fn write_deploy<W: Write>(deploy: &Deploy, mut output: W) -> Result<(), Error> {
    let content =
        serde_json::to_string_pretty(deploy).map_err(|error| Error::FailedToEncodeToJson {
            context: "writing deploy",
            error,
        })?;
    output
        .write_all(content.as_bytes())
        .map_err(|error| Error::IoError {
            context: "unable to write deploy".to_owned(),
            error,
        })
}

#[cfg(any(feature = "std-fs-io", test))]
fn write_transaction<W: Write>(transaction: &TransactionV1, mut output: W) -> Result<(), Error> {
    let content =
        serde_json::to_string_pretty(transaction).map_err(|error| Error::FailedToEncodeToJson {
            context: "writing transaction",
            error,
        })?;
    output
        .write_all(content.as_bytes())
        .map_err(|error| Error::IoError {
            context: "unable to write transaction".to_owned(),
            error,
        })
}

#[cfg(any(feature = "std-fs-io", test))]
fn read_deploy<R: Read>(input: R) -> Result<Deploy, Error> {
    let deploy: Deploy =
        serde_json::from_reader(input).map_err(|error| Error::FailedToDecodeFromJson {
            context: "reading deploy",
            error,
        })?;
    deploy.is_valid_size(MAX_SERIALIZED_SIZE_OF_DEPLOY)?;
    Ok(deploy)
}

#[cfg(any(feature = "std-fs-io", test))]
fn read_transaction<R: Read>(input: R) -> Result<TransactionV1, Error> {
    let transaction: TransactionV1 =
        serde_json::from_reader(input).map_err(|error| Error::FailedToDecodeFromJson {
            context: "reading transaction",
            error,
        })?;
    Ok(transaction)
}

/// Retrieves era information from the network at a given switch [`Block`].
///
/// Sends a JSON-RPC `chain_get_era_info_by_switch_block` request to the specified node.
///
/// For details of the parameters, see [the module docs](crate#common-parameters).  Note that if the
/// specified block is not a switch block then the response will have no era info.
#[deprecated(
    since = "2.0.0",
    note = "prefer 'get_era_summary' as it doesn't require a switch block"
)]
pub async fn get_era_info(
    rpc_id: JsonRpcId,
    node_address: &str,
    verbosity: Verbosity,
    maybe_block_identifier: Option<BlockIdentifier>,
) -> Result<SuccessResponse<GetEraInfoResult>, Error> {
    let params = maybe_block_identifier.map(GetEraInfoParams::new);
    JsonRpcCall::new(rpc_id, node_address, verbosity)
        .send_request(GET_ERA_INFO_METHOD, params)
        .await
}

/// Verifies the smart contract code against the one deployed at given deploy or transaction hash.
#[cfg(any(feature = "std-fs-io", test))]
pub async fn verify_contract(
    hash_str: &str,
    verification_url_base_path: &str,
    project_path: Option<&str>,
    verbosity: Verbosity,
) -> Result<VerificationDetails, Error> {
    if verbosity == Verbosity::Medium || verbosity == Verbosity::High {
        println!("Hash: {hash_str}");
        println!("Verification service base path: {verification_url_base_path}",);
    }

    let project_path = match project_path {
        Some(path) => Path::new(path).to_path_buf(),
        None => match current_dir() {
            Ok(path) => path,
            Err(error) => {
                eprintln!("Cannot get current directory: {error}");
                return Err(Error::ContractVerificationFailed);
            }
        },
    };

    let archive = match build_archive(&project_path) {
        Ok(archive) => {
            if verbosity == Verbosity::Medium || verbosity == Verbosity::High {
                println!("Created project archive (size: {})", archive.len());
            }
            archive
        }
        Err(error) => {
            eprintln!("Cannot create project archive: {error}");
            return Err(Error::ContractVerificationFailed);
        }
    };

    send_verification_request(
        hash_str,
        verification_url_base_path,
        STANDARD.encode(&archive),
        verbosity,
    )
    .await
}
