//! An API suitable for use by a CLI binary.
//!
//! It provides functions and types largely based around strings and integers, as would be expected
//! to be input by a CLI user.  The functions then parse these inputs into the expected Rust types
//! and pass them through to the equivalent API functions defined in the root of the library.
//!
//! # Common Parameters
//!
//! Many of the functions have similar parameters.  Descriptions for these common ones follow:
//!
//! * `maybe_rpc_id` - The JSON-RPC identifier, applied to the request and returned in the response.
//!   If it can be parsed as an `i64` it will be used as a JSON integer. If empty, a random `i64`
//!   will be assigned.  Otherwise the provided string will be used verbatim.
//! * `node_address` - The hostname or IP and port of the server, e.g. `http://127.0.0.1:7777`.
//! * `verbosity_level` - When `1`, the JSON-RPC request will be printed to `stdout` with long
//!   string fields (e.g. hex-formatted raw Wasm bytes) shortened to a string indicating the char
//!   count of the field.  When `verbosity_level` is greater than `1`, the request will be printed
//!   to `stdout` with no abbreviation of long fields.  When `verbosity_level` is `0`, the request
//!   will not be printed to `stdout`.
//! * `maybe_block_id` - Must be a hex-encoded, 32-byte hash digest or a `u64` representing the
//!   [`Block`] height or empty.  If empty, the latest `Block` known on the server will be used.

/// Functions for creating Deploys.
pub mod deploy;
mod deploy_str_params;
mod dictionary_item_str_params;
mod error;
mod json_args;
pub mod parse;
mod payment_str_params;
mod session_str_params;
mod simple_args;
#[cfg(test)]
mod tests;
mod transaction;
mod transaction_builder_params;
mod transaction_str_params;

#[cfg(feature = "std-fs-io")]
use serde::Serialize;

#[cfg(doc)]
use casper_types::{account::AccountHash, Key};

use casper_types::{Digest, URef};

use crate::{
    rpcs::{
        results::{
            GetAccountResult, GetAddressableEntityResult, GetAuctionInfoResult, GetBalanceResult,
            GetBlockResult, GetBlockTransfersResult, GetChainspecResult, GetDeployResult,
            GetDictionaryItemResult, GetEraInfoResult, GetEraSummaryResult, GetNodeStatusResult,
            GetPeersResult, GetRewardResult, GetStateRootHashResult, GetTransactionResult,
            GetValidatorChangesResult, ListRpcsResult, QueryBalanceDetailsResult,
            QueryBalanceResult, QueryGlobalStateResult,
        },
        DictionaryItemIdentifier,
    },
    SuccessResponse,
};

#[cfg(feature = "std-fs-io")]
use crate::verification_types::VerificationDetails;
#[cfg(doc)]
use crate::{Account, Block, Error, StoredValue, Transfer};
#[cfg(doc)]
use casper_types::PublicKey;
#[cfg(feature = "std-fs-io")]
pub use deploy::{
    make_deploy, make_transfer, put_deploy, send_deploy_file, sign_deploy_file,
    speculative_put_deploy, speculative_send_deploy_file, speculative_transfer, transfer,
};
pub use deploy_str_params::DeployStrParams;
pub use dictionary_item_str_params::DictionaryItemStrParams;
pub use error::{CliError, FromDecStrErr};
pub use json_args::{
    help as json_args_help, Error as JsonArgsError, ErrorDetails as JsonArgsErrorDetails, JsonArg,
};
pub use payment_str_params::PaymentStrParams;
pub use session_str_params::SessionStrParams;
pub use simple_args::{help as simple_args_help, insert_arg};
pub use transaction::{make_transaction, put_transaction};
#[cfg(feature = "std-fs-io")]
pub use transaction::{
    send_transaction_file, sign_transaction_file, speculative_send_transaction_file,
};
pub use transaction_builder_params::TransactionBuilderParams;
pub use transaction_str_params::TransactionStrParams;

/// Retrieves a [`casper_types::Deploy`] from the network.
///
/// `deploy_hash` must be a hex-encoded, 32-byte hash digest.  For details of the other parameters,
/// see [the module docs](crate::cli#common-parameters).
pub async fn get_deploy(
    maybe_rpc_id: &str,
    node_address: &str,
    verbosity_level: u64,
    deploy_hash: &str,
    finalized_approvals: bool,
) -> Result<SuccessResponse<GetDeployResult>, CliError> {
    let rpc_id = parse::rpc_id(maybe_rpc_id);
    let verbosity = parse::verbosity(verbosity_level);
    let deploy_hash = parse::deploy_hash(deploy_hash)?;
    crate::get_deploy(
        rpc_id,
        node_address,
        verbosity,
        deploy_hash,
        finalized_approvals,
    )
    .await
    .map_err(CliError::from)
}

/// Retrieves a [`casper_types::Transaction`] from the network.
///
/// `transaction_hash` must be a hex-encoded, 32-byte hash digest.  For details of the other parameters,
/// see [the module docs](crate::cli#common-parameters).
pub async fn get_transaction(
    maybe_rpc_id: &str,
    node_address: &str,
    verbosity_level: u64,
    transaction_hash: &str,
    finalized_approvals: bool,
) -> Result<SuccessResponse<GetTransactionResult>, CliError> {
    let rpc_id = parse::rpc_id(maybe_rpc_id);
    let verbosity = parse::verbosity(verbosity_level);
    let transaction_hash = parse::transaction_hash(transaction_hash)?;
    crate::get_transaction(
        rpc_id,
        node_address,
        verbosity,
        transaction_hash,
        finalized_approvals,
    )
    .await
    .map_err(CliError::from)
}
/// Retrieves a [`Block`] from the network.
///
/// For details of the parameters, see [the module docs](crate::cli#common-parameters).
pub async fn get_block(
    maybe_rpc_id: &str,
    node_address: &str,
    verbosity_level: u64,
    maybe_block_id: &str,
) -> Result<SuccessResponse<GetBlockResult>, CliError> {
    let rpc_id = parse::rpc_id(maybe_rpc_id);
    let verbosity = parse::verbosity(verbosity_level);
    let maybe_block_id = parse::block_identifier(maybe_block_id)?;
    crate::get_block(rpc_id, node_address, verbosity, maybe_block_id)
        .await
        .map_err(CliError::from)
}

/// Retrieves all [`Transfer`] items for a [`Block`] from the network.
///
/// For details of the parameters, see [the module docs](crate::cli#common-parameters).
pub async fn get_block_transfers(
    maybe_rpc_id: &str,
    node_address: &str,
    verbosity_level: u64,
    maybe_block_id: &str,
) -> Result<SuccessResponse<GetBlockTransfersResult>, CliError> {
    let rpc_id = parse::rpc_id(maybe_rpc_id);
    let verbosity = parse::verbosity(verbosity_level);
    let maybe_block_id = parse::block_identifier(maybe_block_id)?;
    crate::get_block_transfers(rpc_id, node_address, verbosity, maybe_block_id)
        .await
        .map_err(CliError::from)
}

/// Retrieves a state root hash at a given [`Block`].
///
/// For details of the parameters, see [the module docs](crate::cli#common-parameters).
pub async fn get_state_root_hash(
    maybe_rpc_id: &str,
    node_address: &str,
    verbosity_level: u64,
    maybe_block_id: &str,
) -> Result<SuccessResponse<GetStateRootHashResult>, CliError> {
    let rpc_id = parse::rpc_id(maybe_rpc_id);
    let verbosity = parse::verbosity(verbosity_level);
    let maybe_block_id = parse::block_identifier(maybe_block_id)?;
    crate::get_state_root_hash(rpc_id, node_address, verbosity, maybe_block_id)
        .await
        .map_err(CliError::from)
}

/// Retrieves era information from the network at a given [`Block`].
///
/// For details of the parameters, see [the module docs](crate::cli#common-parameters).
pub async fn get_era_summary(
    maybe_rpc_id: &str,
    node_address: &str,
    verbosity_level: u64,
    maybe_block_id: &str,
) -> Result<SuccessResponse<GetEraSummaryResult>, CliError> {
    let rpc_id = parse::rpc_id(maybe_rpc_id);
    let verbosity = parse::verbosity(verbosity_level);
    let maybe_block_id = parse::block_identifier(maybe_block_id)?;
    crate::get_era_summary(rpc_id, node_address, verbosity, maybe_block_id)
        .await
        .map_err(CliError::from)
}

/// Retrieves a [`StoredValue`] from global state.
///
/// `maybe_block_id` or `maybe_state_root_hash` identify the global state root hash to be used for
/// the query.  Exactly one of these args should be an empty string.
///
/// `key` must be a formatted [`PublicKey`] or [`Key`].  `path` is comprised of components starting
/// from the `key`, separated by `/`s.  It may be empty.
///
/// For details of other parameters, see [the module docs](crate::cli#common-parameters).
pub async fn query_global_state(
    maybe_rpc_id: &str,
    node_address: &str,
    verbosity_level: u64,
    maybe_block_id: &str,
    maybe_state_root_hash: &str,
    key: &str,
    path: &str,
) -> Result<SuccessResponse<QueryGlobalStateResult>, CliError> {
    let rpc_id = parse::rpc_id(maybe_rpc_id);
    let verbosity = parse::verbosity(verbosity_level);
    let global_state_identifier =
        parse::global_state_identifier(maybe_block_id, maybe_state_root_hash)?
            .ok_or(CliError::FailedToParseStateIdentifier)?;
    let key = parse::key_for_query(key)?;
    let path = if path.is_empty() {
        vec![]
    } else {
        path.split('/').map(ToString::to_string).collect()
    };

    crate::query_global_state(
        rpc_id,
        node_address,
        verbosity,
        global_state_identifier,
        key,
        path,
    )
    .await
    .map_err(CliError::from)
}

/// Retrieves a purse's balance from global state.
///
/// `maybe_block_id` or `maybe_state_root_hash` identify the global state root hash to be used for
/// the query.  If both are empty, the latest block is used.
///
/// `purse_id` can be a properly-formatted public key, account hash, entity address or URef.
///
/// For details of other parameters, see [the module docs](crate::cli#common-parameters).
pub async fn query_balance(
    maybe_rpc_id: &str,
    node_address: &str,
    verbosity_level: u64,
    maybe_block_id: &str,
    maybe_state_root_hash: &str,
    purse_id: &str,
) -> Result<SuccessResponse<QueryBalanceResult>, CliError> {
    let rpc_id = parse::rpc_id(maybe_rpc_id);
    let verbosity = parse::verbosity(verbosity_level);
    let maybe_global_state_identifier =
        parse::global_state_identifier(maybe_block_id, maybe_state_root_hash)?;
    let purse_identifier = parse::purse_identifier(purse_id)?;

    crate::query_balance(
        rpc_id,
        node_address,
        verbosity,
        maybe_global_state_identifier,
        purse_identifier,
    )
    .await
    .map_err(CliError::from)
}

/// Retrieves a purse's balance and hold information from global state.
///
/// `maybe_block_id` or `maybe_state_root_hash` identify the global state root hash to be used for
/// the query.  If both are empty, the latest block is used.
///
/// `purse_id` can be a properly-formatted public key, account hash, entity address or URef.
///
/// For details of other parameters, see [the module docs](crate::cli#common-parameters).
pub async fn query_balance_details(
    maybe_rpc_id: &str,
    node_address: &str,
    verbosity_level: u64,
    maybe_block_id: &str,
    maybe_state_root_hash: &str,
    purse_id: &str,
) -> Result<SuccessResponse<QueryBalanceDetailsResult>, CliError> {
    let rpc_id = parse::rpc_id(maybe_rpc_id);
    let verbosity = parse::verbosity(verbosity_level);
    let maybe_global_state_identifier =
        parse::global_state_identifier(maybe_block_id, maybe_state_root_hash)?;
    let purse_identifier = parse::purse_identifier(purse_id)?;

    crate::query_balance_details(
        rpc_id,
        node_address,
        verbosity,
        maybe_global_state_identifier,
        purse_identifier,
    )
    .await
    .map_err(CliError::from)
}

/// Retrieves a [`StoredValue`] from a dictionary at a given state root hash.
///
/// `state_root_hash` must be a hex-encoded, 32-byte hash digest.
///
/// `dictionary_item_str_params` contains dictionary item identifier options for this query.  See
/// [`DictionaryItemStrParams`] for more details.
///
/// For details of other parameters, see [the module docs](crate::cli#common-parameters).
pub async fn get_dictionary_item(
    maybe_rpc_id: &str,
    node_address: &str,
    verbosity_level: u64,
    state_root_hash: &str,
    dictionary_item_str_params: DictionaryItemStrParams<'_>,
) -> Result<SuccessResponse<GetDictionaryItemResult>, CliError> {
    let rpc_id = parse::rpc_id(maybe_rpc_id);
    let verbosity = parse::verbosity(verbosity_level);
    let state_root_hash =
        Digest::from_hex(state_root_hash).map_err(|error| CliError::FailedToParseDigest {
            context: "state root hash in get_dictionary_item",
            error,
        })?;
    let dictionary_item_identifier =
        DictionaryItemIdentifier::try_from(dictionary_item_str_params)?;

    crate::get_dictionary_item(
        rpc_id,
        node_address,
        verbosity,
        state_root_hash,
        dictionary_item_identifier,
    )
    .await
    .map_err(CliError::from)
}

/// Retrieves a purse's balance at a given state root hash.
///
/// `state_root_hash` must be a hex-encoded, 32-byte hash digest.
///
/// `purse` is a URef, formatted as e.g.
/// ```text
/// uref-0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20-007
/// ```
///
/// For details of other parameters, see [the module docs](crate::cli#common-parameters).
pub async fn get_balance(
    maybe_rpc_id: &str,
    node_address: &str,
    verbosity_level: u64,
    state_root_hash: &str,
    purse: &str,
) -> Result<SuccessResponse<GetBalanceResult>, CliError> {
    let rpc_id = parse::rpc_id(maybe_rpc_id);
    let verbosity = parse::verbosity(verbosity_level);
    let state_root_hash =
        Digest::from_hex(state_root_hash).map_err(|error| CliError::FailedToParseDigest {
            context: "state root hash in get_balance",
            error,
        })?;
    let purse = URef::from_formatted_str(purse).map_err(|error| CliError::FailedToParseURef {
        context: "purse in get_balance",
        error,
    })?;

    crate::get_balance(rpc_id, node_address, verbosity, state_root_hash, purse)
        .await
        .map_err(CliError::from)
}

/// Retrieves an [`Account`] at a given [`Block`].
///
/// For details of other parameters, see [the module docs](crate::cli#common-parameters).
///
/// # Parameters
/// - `maybe_rpc_id`: The optional RPC ID as a string slice.
/// - `node_address`: The address of the node as a string slice.
/// - `verbosity_level`: The verbosity level as a 64-bit unsigned integer.
/// - `maybe_block_id`: The optional block ID as a string slice.
/// - `account_identifier`: The account identifier as a string slice.
///
/// # Returns
/// The result containing either a successful response with the account details or a `CliError`.

pub async fn get_account(
    maybe_rpc_id: &str,
    node_address: &str,
    verbosity_level: u64,
    maybe_block_id: &str,
    account_identifier: &str,
) -> Result<SuccessResponse<GetAccountResult>, CliError> {
    let rpc_id = parse::rpc_id(maybe_rpc_id);
    let verbosity = parse::verbosity(verbosity_level);
    let maybe_block_id = parse::block_identifier(maybe_block_id)?;
    let account_identifier = parse::account_identifier(account_identifier)?;

    crate::get_account(
        rpc_id,
        node_address,
        verbosity,
        maybe_block_id,
        account_identifier,
    )
    .await
    .map_err(CliError::from)
}

/// Retrieves an [`crate::rpcs::v2_0_0::get_entity::EntityOrAccount`] at a given [`Block`].
///
/// For details of other parameters, see [the module docs](crate::cli#common-parameters).
///
/// # Parameters
/// - `maybe_rpc_id`: The optional RPC ID as a string slice.
/// - `node_address`: The address of the node as a string slice.
/// - `verbosity_level`: The verbosity level as a 64-bit unsigned integer.
/// - `maybe_block_id`: The optional block ID as a string slice.
/// - `entity_identifier`: The entity identifier as a string slice.
///
/// # Returns
/// The result containing either a successful response with the entity details or a `CliError`.
pub async fn get_entity(
    maybe_rpc_id: &str,
    node_address: &str,
    verbosity_level: u64,
    maybe_block_id: &str,
    entity_identifier: &str,
) -> Result<SuccessResponse<GetAddressableEntityResult>, CliError> {
    let rpc_id = parse::rpc_id(maybe_rpc_id);
    let verbosity = parse::verbosity(verbosity_level);
    let maybe_block_id = parse::block_identifier(maybe_block_id)?;
    let entity_identifier = parse::entity_identifier(entity_identifier)?;

    crate::get_entity(
        rpc_id,
        node_address,
        verbosity,
        maybe_block_id,
        entity_identifier,
    )
    .await
    .map_err(CliError::from)
}

/// Retrieves an [`GetRewardResult`] at a given era.
///
/// `validator` is the public key as a formatted string associated with the validator.
///
/// `maybe_delegator` is the public key as a formatted string associated with the delegator.
///
/// For details of other parameters, see [the module docs](crate::cli#common-parameters).
pub async fn get_reward(
    maybe_rpc_id: &str,
    node_address: &str,
    verbosity_level: u64,
    maybe_era_id: &str,
    validator: &str,
    maybe_delegator: &str,
) -> Result<SuccessResponse<GetRewardResult>, CliError> {
    let rpc_id = parse::rpc_id(maybe_rpc_id);
    let verbosity = parse::verbosity(verbosity_level);
    let era_identifier = parse::era_identifier(maybe_era_id)?;
    let validator =
        parse::public_key(validator)?.ok_or(CliError::FailedToParseValidatorPublicKey)?;
    let delegator = parse::public_key(maybe_delegator)?;

    crate::get_reward(
        rpc_id,
        node_address,
        verbosity,
        era_identifier,
        validator,
        delegator,
    )
    .await
    .map_err(CliError::from)
}

/// Retrieves the bids and validators at a given [`Block`].
///
/// For details of the parameters, see [the module docs](crate::cli#common-parameters).
pub async fn get_auction_info(
    maybe_rpc_id: &str,
    node_address: &str,
    verbosity_level: u64,
    maybe_block_id: &str,
) -> Result<SuccessResponse<GetAuctionInfoResult>, CliError> {
    let rpc_id = parse::rpc_id(maybe_rpc_id);
    let verbosity = parse::verbosity(verbosity_level);
    let maybe_block_id = parse::block_identifier(maybe_block_id)?;
    crate::get_auction_info(rpc_id, node_address, verbosity, maybe_block_id)
        .await
        .map_err(CliError::from)
}

/// Retrieves the status changes of the active validators on the network.
///
/// For details of the parameters, see [the module docs](crate::cli#common-parameters).
pub async fn get_validator_changes(
    maybe_rpc_id: &str,
    node_address: &str,
    verbosity_level: u64,
) -> Result<SuccessResponse<GetValidatorChangesResult>, CliError> {
    let rpc_id = parse::rpc_id(maybe_rpc_id);
    let verbosity = parse::verbosity(verbosity_level);
    crate::get_validator_changes(rpc_id, node_address, verbosity)
        .await
        .map_err(CliError::from)
}

/// Retrieves the IDs and addresses of the specified node's peers.
///
/// For details of the parameters, see [the module docs](crate::cli#common-parameters).
pub async fn get_peers(
    maybe_rpc_id: &str,
    node_address: &str,
    verbosity_level: u64,
) -> Result<SuccessResponse<GetPeersResult>, CliError> {
    let rpc_id = parse::rpc_id(maybe_rpc_id);
    let verbosity = parse::verbosity(verbosity_level);
    crate::get_peers(rpc_id, node_address, verbosity)
        .await
        .map_err(CliError::from)
}

/// Retrieves the status of the specified node.
///
/// For details of the parameters, see [the module docs](crate::cli#common-parameters).
pub async fn get_node_status(
    maybe_rpc_id: &str,
    node_address: &str,
    verbosity_level: u64,
) -> Result<SuccessResponse<GetNodeStatusResult>, CliError> {
    let rpc_id = parse::rpc_id(maybe_rpc_id);
    let verbosity = parse::verbosity(verbosity_level);
    crate::get_node_status(rpc_id, node_address, verbosity)
        .await
        .map_err(CliError::from)
}

/// Retrieves the Chainspec of the network.
///
/// For details of the parameters, see [the module docs](crate::cli#common-parameters).
pub async fn get_chainspec(
    maybe_rpc_id: &str,
    node_address: &str,
    verbosity_level: u64,
) -> Result<SuccessResponse<GetChainspecResult>, CliError> {
    let rpc_id = parse::rpc_id(maybe_rpc_id);
    let verbosity = parse::verbosity(verbosity_level);
    crate::get_chainspec(rpc_id, node_address, verbosity)
        .await
        .map_err(CliError::from)
}

/// Retrieves the interface description (the schema including examples in OpenRPC format) of the
/// JSON-RPC server's API.
///
/// For details of the parameters, see [the module docs](crate::cli#common-parameters).
pub async fn list_rpcs(
    maybe_rpc_id: &str,
    node_address: &str,
    verbosity_level: u64,
) -> Result<SuccessResponse<ListRpcsResult>, CliError> {
    let rpc_id = parse::rpc_id(maybe_rpc_id);
    let verbosity = parse::verbosity(verbosity_level);
    crate::list_rpcs(rpc_id, node_address, verbosity)
        .await
        .map_err(CliError::from)
}

/// JSON-encode and pretty-print the given value to stdout at the given verbosity level.
///
/// When `verbosity_level` is `0`, nothing is printed.  For `1`, the value is printed with long
/// string fields shortened to a string indicating the character count of the field.  Greater than
/// `1` is the same as for `1` except without abbreviation of long fields.
#[cfg(feature = "std-fs-io")]
pub fn json_pretty_print<T: ?Sized + Serialize>(
    value: &T,
    verbosity_level: u64,
) -> Result<(), CliError> {
    let verbosity = parse::verbosity(verbosity_level);
    crate::json_pretty_print(value, verbosity).map_err(CliError::from)
}

/// Retrieves era information from the network at a given switch [`Block`].
///
/// For details of the parameters, see [the module docs](crate::cli#common-parameters).
#[deprecated(
    since = "2.0.0",
    note = "prefer 'get_era_summary' as it doesn't require a switch block"
)]
pub async fn get_era_info(
    maybe_rpc_id: &str,
    node_address: &str,
    verbosity_level: u64,
    maybe_block_id: &str,
) -> Result<SuccessResponse<GetEraInfoResult>, CliError> {
    let rpc_id = parse::rpc_id(maybe_rpc_id);
    let verbosity = parse::verbosity(verbosity_level);
    let maybe_block_id = parse::block_identifier(maybe_block_id)?;
    #[allow(deprecated)]
    crate::get_era_info(rpc_id, node_address, verbosity, maybe_block_id)
        .await
        .map_err(CliError::from)
}

/// Verifies the smart contract code against the one installed
/// by deploy or transaction with given hash.
#[cfg(feature = "std-fs-io")]
pub async fn verify_contract(
    hash_str: &str,
    verification_url_base_path: &str,
    verification_project_path: Option<&str>,
    verbosity_level: u64,
) -> Result<VerificationDetails, CliError> {
    let verbosity = parse::verbosity(verbosity_level);
    crate::verify_contract(
        hash_str,
        verification_url_base_path,
        verification_project_path,
        verbosity,
    )
    .await
    .map_err(CliError::from)
}
