use casper_types::{
    account::AccountHash, AsymmetricType, Deploy, DeployBuilder, PublicKey, TransferTarget,
    UIntParseError, URef, U512,
};

use super::{parse, CliError, DeployStrParams, PaymentStrParams, SessionStrParams};
use crate::rpcs::results::{PutDeployResult, SpeculativeExecResult};
use crate::{SuccessResponse, MAX_SERIALIZED_SIZE_OF_DEPLOY};

const DEFAULT_GAS_PRICE: u64 = 1;

/// Creates a [`Deploy`] and sends it to the network for execution.
///
/// For details of the parameters, see [the module docs](crate::cli#common-parameters) or the docs
/// of the individual parameter types.
pub async fn put_deploy(
    maybe_rpc_id: &str,
    node_address: &str,
    verbosity_level: u64,
    deploy_params: DeployStrParams<'_>,
    session_params: SessionStrParams<'_>,
    payment_params: PaymentStrParams<'_>,
) -> Result<SuccessResponse<PutDeployResult>, CliError> {
    let rpc_id = parse::rpc_id(maybe_rpc_id);
    let verbosity = parse::verbosity(verbosity_level);
    let deploy = with_payment_and_session(deploy_params, payment_params, session_params, false)?;
    crate::put_deploy(rpc_id, node_address, verbosity, deploy)
        .await
        .map_err(CliError::from)
}

/// Creates a [`Deploy`] and sends it to the specified node for speculative execution.
///
/// For details of the parameters, see [the module docs](crate::cli#common-parameters) or the docs
/// of the individual parameter types.
pub async fn speculative_put_deploy(
    maybe_block_id: &str,
    maybe_rpc_id: &str,
    node_address: &str,
    verbosity_level: u64,
    deploy_params: DeployStrParams<'_>,
    session_params: SessionStrParams<'_>,
    payment_params: PaymentStrParams<'_>,
) -> Result<SuccessResponse<SpeculativeExecResult>, CliError> {
    let rpc_id = parse::rpc_id(maybe_rpc_id);
    let verbosity = parse::verbosity(verbosity_level);
    let deploy = with_payment_and_session(deploy_params, payment_params, session_params, false)?;
    let speculative_exec = parse::block_identifier(maybe_block_id)?;
    crate::speculative_exec(rpc_id, node_address, speculative_exec, verbosity, deploy)
        .await
        .map_err(CliError::from)
}
/// Creates a [`Deploy`] and outputs it to a file or stdout.
///
/// As a file, the `Deploy` can subsequently be signed by other parties using [`sign_deploy_file`]
/// and then sent to the network for execution using [`send_deploy_file`].
///
/// `maybe_output_path` specifies the output file path, or if empty, will print it to `stdout`.  If
/// `force` is true, and a file exists at `maybe_output_path`, it will be overwritten.  If `force`
/// is false and a file exists at `maybe_output_path`, [`Error::FileAlreadyExists`] is returned
/// and the file will not be written.
pub fn make_deploy(
    maybe_output_path: &str,
    deploy_params: DeployStrParams<'_>,
    session_params: SessionStrParams<'_>,
    payment_params: PaymentStrParams<'_>,
    force: bool,
) -> Result<(), CliError> {
    let output = parse::output_kind(maybe_output_path, force);
    let deploy = with_payment_and_session(deploy_params, payment_params, session_params, true)?;
    crate::output_deploy(output, &deploy).map_err(CliError::from)
}

/// Reads a previously-saved [`Deploy`] from a file, cryptographically signs it, and outputs it to a
/// file or stdout.
///
/// `maybe_output_path` specifies the output file path, or if empty, will print it to `stdout`.  If
/// `force` is true, and a file exists at `maybe_output_path`, it will be overwritten.  If `force`
/// is false and a file exists at `maybe_output_path`, [`Error::FileAlreadyExists`] is returned
/// and the file will not be written.
pub fn sign_deploy_file(
    input_path: &str,
    secret_key_path: &str,
    maybe_output_path: &str,
    force: bool,
) -> Result<(), CliError> {
    let secret_key = parse::secret_key_from_file(secret_key_path)?;
    let output = parse::output_kind(maybe_output_path, force);
    crate::sign_deploy_file(input_path, &secret_key, output).map_err(CliError::from)
}

/// Reads a previously-saved [`Deploy`] from a file and sends it to the network for execution.
///
/// For details of the parameters, see [the module docs](crate::cli#common-parameters).
pub async fn send_deploy_file(
    maybe_rpc_id: &str,
    node_address: &str,
    verbosity_level: u64,
    input_path: &str,
) -> Result<SuccessResponse<PutDeployResult>, CliError> {
    let rpc_id = parse::rpc_id(maybe_rpc_id);
    let verbosity = parse::verbosity(verbosity_level);
    let deploy = crate::read_deploy_file(input_path)?;
    crate::put_deploy(rpc_id, node_address, verbosity, deploy)
        .await
        .map_err(CliError::from)
}

/// Reads a previously-saved [`Deploy`] from a file and sends it to the specified node for
/// speculative execution.
/// For details of the parameters, see [the module docs](crate::cli#common-parameters).
pub async fn speculative_send_deploy_file(
    maybe_block_id: &str,
    maybe_rpc_id: &str,
    node_address: &str,
    verbosity_level: u64,
    input_path: &str,
) -> Result<SuccessResponse<SpeculativeExecResult>, CliError> {
    let rpc_id = parse::rpc_id(maybe_rpc_id);
    let speculative_exec = parse::block_identifier(maybe_block_id)?;
    let verbosity = parse::verbosity(verbosity_level);
    let deploy = crate::read_deploy_file(input_path)?;
    crate::speculative_exec(rpc_id, node_address, speculative_exec, verbosity, deploy)
        .await
        .map_err(CliError::from)
}

/// Transfers funds between purses.
///
/// * `amount` is a string to be parsed as a `U512` specifying the amount to be transferred.
/// * `target_account` is the [`AccountHash`], [`URef`] or [`PublicKey`] of the account to which the
///   funds will be transferred, formatted as a hex-encoded string.  The account's main purse will
///   receive the funds.
/// * `transfer_id` is a string to be parsed as a `u64` representing a user-defined identifier which
///   will be permanently associated with the transfer.
///
/// For details of other parameters, see [the module docs](crate::cli#common-parameters).
#[allow(clippy::too_many_arguments)]
pub async fn transfer(
    maybe_rpc_id: &str,
    node_address: &str,
    verbosity_level: u64,
    amount: &str,
    target_account: &str,
    transfer_id: &str,
    deploy_params: DeployStrParams<'_>,
    payment_params: PaymentStrParams<'_>,
) -> Result<SuccessResponse<PutDeployResult>, CliError> {
    let rpc_id = parse::rpc_id(maybe_rpc_id);
    let verbosity = parse::verbosity(verbosity_level);
    let deploy = new_transfer(
        amount,
        None,
        target_account,
        transfer_id,
        deploy_params,
        payment_params,
        false,
    )?;
    crate::put_deploy(rpc_id, node_address, verbosity, deploy)
        .await
        .map_err(CliError::from)
}

/// Creates a [`Deploy`] to transfer funds between purses, and sends it to the specified node for
/// speculative execution.
///
/// * `amount` is a string to be parsed as a `U512` specifying the amount to be transferred.
/// * `target_account` is the [`AccountHash`], [`URef`] or [`PublicKey`] of the account to which the
///   funds will be transferred, formatted as a hex-encoded string.  The account's main purse will
///   receive the funds.
/// * `transfer_id` is a string to be parsed as a `u64` representing a user-defined identifier which
///   will be permanently associated with the transfer.
///
/// For details of other parameters, see [the module docs](crate::cli#common-parameters).
#[allow(clippy::too_many_arguments)]
pub async fn speculative_transfer(
    maybe_block_id: &str,
    maybe_rpc_id: &str,
    node_address: &str,
    verbosity_level: u64,
    amount: &str,
    target_account: &str,
    transfer_id: &str,
    deploy_params: DeployStrParams<'_>,
    payment_params: PaymentStrParams<'_>,
) -> Result<SuccessResponse<SpeculativeExecResult>, CliError> {
    let rpc_id = parse::rpc_id(maybe_rpc_id);
    let verbosity = parse::verbosity(verbosity_level);
    let deploy = new_transfer(
        amount,
        None,
        target_account,
        transfer_id,
        deploy_params,
        payment_params,
        false,
    )?;
    let speculative_exec = parse::block_identifier(maybe_block_id)?;
    crate::speculative_exec(rpc_id, node_address, speculative_exec, verbosity, deploy)
        .await
        .map_err(CliError::from)
}

/// Creates a transfer [`Deploy`] and outputs it to a file or stdout.
///
/// As a file, the `Deploy` can subsequently be signed by other parties using [`sign_deploy_file`]
/// and then sent to the network for execution using [`send_deploy_file`].
///
/// `maybe_output_path` specifies the output file path, or if empty, will print it to `stdout`.  If
/// `force` is true, and a file exists at `maybe_output_path`, it will be overwritten.  If `force`
/// is false and a file exists at `maybe_output_path`, [`Error::FileAlreadyExists`] is returned
/// and the file will not be written.
pub fn make_transfer(
    maybe_output_path: &str,
    amount: &str,
    target_account: &str,
    transfer_id: &str,
    deploy_params: DeployStrParams<'_>,
    payment_params: PaymentStrParams<'_>,
    force: bool,
) -> Result<(), CliError> {
    let output = parse::output_kind(maybe_output_path, force);
    let deploy = new_transfer(
        amount,
        None,
        target_account,
        transfer_id,
        deploy_params,
        payment_params,
        true,
    )?;
    crate::output_deploy(output, &deploy).map_err(CliError::from)
}

/// Creates new Deploy with specified payment and session data.
pub fn with_payment_and_session(
    deploy_params: DeployStrParams,
    payment_params: PaymentStrParams,
    session_params: SessionStrParams,
    allow_unsigned_deploy: bool,
) -> Result<Deploy, CliError> {
    let gas_price: u64 = deploy_params.gas_price_tolerance.parse::<u64>().unwrap_or(DEFAULT_GAS_PRICE);
    let chain_name = deploy_params.chain_name.to_string();
    let session = parse::session_executable_deploy_item(session_params)?;
    let maybe_secret_key = if allow_unsigned_deploy && deploy_params.secret_key.is_empty() {
        None
    } else if deploy_params.secret_key.is_empty() && !allow_unsigned_deploy {
        return Err(CliError::InvalidArgument {
            context: "with_payment_and_session (secret_key, allow_unsigned_deploy)",
            error: format!(
                "allow_unsigned_deploy was {}, but no secret key was provided",
                allow_unsigned_deploy
            ),
        });
    } else {
        Some(parse::secret_key_from_file(deploy_params.secret_key)?)
    };
    let payment = parse::payment_executable_deploy_item(payment_params)?;
    let timestamp = parse::timestamp(deploy_params.timestamp)?;
    let ttl = parse::ttl(deploy_params.ttl)?;
    let maybe_session_account = parse::session_account(deploy_params.session_account)?;

    let mut deploy_builder = DeployBuilder::new(chain_name, session)
        .with_payment(payment)
        .with_timestamp(timestamp)
        .with_gas_price(gas_price)
        .with_ttl(ttl);

    if let Some(secret_key) = &maybe_secret_key {
        deploy_builder = deploy_builder.with_secret_key(secret_key);
    }
    if let Some(account) = maybe_session_account {
        deploy_builder = deploy_builder.with_account(account);
    }

    let deploy = deploy_builder.build().map_err(crate::Error::from)?;
    deploy
        .is_valid_size(MAX_SERIALIZED_SIZE_OF_DEPLOY)
        .map_err(crate::Error::from)?;
    Ok(deploy)
}

/// Creates new Transfer with specified data.
pub fn new_transfer(
    amount: &str,
    source_purse: Option<URef>,
    target_account: &str,
    transfer_id: &str,
    deploy_params: DeployStrParams,
    payment_params: PaymentStrParams,
    allow_unsigned_deploy: bool,
) -> Result<Deploy, CliError> {
    let chain_name = deploy_params.chain_name.to_string();
    let maybe_secret_key = if allow_unsigned_deploy && deploy_params.secret_key.is_empty() {
        None
    } else if deploy_params.secret_key.is_empty() && !allow_unsigned_deploy {
        return Err(CliError::InvalidArgument {
            context: "new_transfer (secret_key, allow_unsigned_deploy)",
            error: format!(
                "allow_unsigned_deploy was {}, but no secret key was provided",
                allow_unsigned_deploy
            ),
        });
    } else {
        Some(parse::secret_key_from_file(deploy_params.secret_key)?)
    };
    let payment = parse::payment_executable_deploy_item(payment_params)?;

    let amount = U512::from_dec_str(amount).map_err(|err| CliError::FailedToParseUint {
        context: "new_transfer amount",
        error: UIntParseError::FromDecStr(err),
    })?;

    let target = if let Ok(public_key) = PublicKey::from_hex(target_account) {
        TransferTarget::PublicKey(public_key)
    } else if let Ok(account_hash) = AccountHash::from_formatted_str(target_account) {
        TransferTarget::AccountHash(account_hash)
    } else if let Ok(uref) = URef::from_formatted_str(target_account) {
        TransferTarget::URef(uref)
    } else {
        return Err(CliError::InvalidArgument {
            context: "new_transfer target_account",
            error: format!(
                "allowed types: PublicKey, AccountHash or URef, got {}",
                target_account
            ),
        });
    };

    let transfer_id = parse::transfer_id(transfer_id)?;
    let maybe_transfer_id = Some(transfer_id);

    let timestamp = parse::timestamp(deploy_params.timestamp)?;
    let ttl = parse::ttl(deploy_params.ttl)?;
    let maybe_session_account = parse::session_account(deploy_params.session_account)?;
    let gas_price: u64 = deploy_params.gas_price_tolerance.parse::<u64>().unwrap_or(DEFAULT_GAS_PRICE);

    let mut deploy_builder =
        DeployBuilder::new_transfer(chain_name, amount, source_purse, target, maybe_transfer_id)
            .with_payment(payment)
            .with_timestamp(timestamp)
            .with_gas_price(gas_price)
            .with_ttl(ttl);
    if let Some(secret_key) = &maybe_secret_key {
        deploy_builder = deploy_builder.with_secret_key(secret_key);
    }
    if let Some(account) = maybe_session_account {
        deploy_builder = deploy_builder.with_account(account);
    }
    let deploy = deploy_builder.build().map_err(crate::Error::from)?;
    deploy
        .is_valid_size(MAX_SERIALIZED_SIZE_OF_DEPLOY)
        .map_err(crate::Error::from)?;
    Ok(deploy)
}
