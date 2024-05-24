use crate::rpcs::v2_0_0::speculative_exec_transaction::SpeculativeExecTxnResult;
use crate::{
    cli::{parse, CliError, TransactionBuilderParams, TransactionStrParams},
    put_transaction as put_transaction_rpc_handler, read_transaction_file,
    rpcs::results::PutTransactionResult,
    speculative_exec_txn, SuccessResponse,
};
use casper_types::{
    Digest, InitiatorAddr, Transaction, TransactionSessionKind, TransactionV1, TransactionV1Builder};

pub fn create_transaction(
    builder_params: TransactionBuilderParams,
    transaction_params: TransactionStrParams,
    allow_unsigned_transaction: bool,
) -> Result<TransactionV1, CliError> {
    let chain_name = transaction_params.chain_name.to_string();
    if transaction_params.payment_amount.is_empty() {
        return Err(CliError::InvalidArgument {
            context: "create_transaction (payment_amount)",
            error: "payment_amount is required to be non empty".to_string(),
        });
    }

    let maybe_secret_key = if allow_unsigned_transaction && transaction_params.secret_key.is_empty()
    {
        None
    } else if transaction_params.secret_key.is_empty() && !allow_unsigned_transaction {
        return Err(CliError::InvalidArgument {
            context: "create_transaction (secret_key, allow_unsigned_deploy)",
            error: format!(
                "allow_unsigned_deploy was {}, but no secret key was provided",
                allow_unsigned_transaction
            ),
        });
    } else {
        Some(parse::secret_key_from_file(transaction_params.secret_key)?)
    };

    let timestamp = parse::timestamp(transaction_params.timestamp)?;
    let ttl = parse::ttl(transaction_params.ttl)?;
    let maybe_session_account = parse::session_account(&transaction_params.initiator_addr)?;

    let mut transaction_builder = make_transaction_builder(builder_params)?;

    transaction_builder = transaction_builder
        .with_timestamp(timestamp)
        .with_ttl(ttl)
        .with_chain_name(chain_name);

    if transaction_params.pricing_mode.is_empty() {
        return Err(CliError::InvalidArgument {
            context: "create_transaction (pricing_mode)",
            error: "pricing_mode is required to be non empty".to_string(),
        });
    }
    let pricing_mode = if transaction_params.payment_amount.to_lowercase().as_str() == "reserved" {
        let digest = Digest::from_hex(transaction_params.receipt).map_err(|error| {
            CliError::FailedToParseDigest {
                context: "pricing_digest",
                error,
            }
        })?;

        parse::pricing_mode(
            transaction_params.pricing_mode,
            transaction_params.payment_amount,
            transaction_params.gas_price_tolerance,
            transaction_params.standard_payment,
            Some(digest),
        )?
    } else {
        parse::pricing_mode(
            transaction_params.pricing_mode,
            transaction_params.payment_amount,
            transaction_params.gas_price_tolerance,
            transaction_params.standard_payment,
            None,
        )?
    };

    transaction_builder = transaction_builder.with_pricing_mode(pricing_mode);

    let maybe_json_args = parse::args_json::session::parse(transaction_params.session_args_json)?;
    let maybe_simple_args =
        parse::arg_simple::session::parse(&transaction_params.session_args_simple)?;

    let args = parse::args_from_simple_or_json(maybe_simple_args, maybe_json_args);
    if !args.is_empty() {
        transaction_builder = transaction_builder.with_runtime_args(args);
    }
    if let Some(secret_key) = &maybe_secret_key {
        transaction_builder = transaction_builder.with_secret_key(secret_key);
    }

    if let Some(account) = maybe_session_account {
        transaction_builder =
            transaction_builder.with_initiator_addr(InitiatorAddr::PublicKey(account));
    }

    let txn = transaction_builder.build().map_err(crate::Error::from)?;
    Ok(txn)
}

/// Creates a [`Transaction`] and outputs it to a file or stdout.
///
/// As a file, the `Transaction` can subsequently be signed by other parties using [`sign_transaction_file`]
/// and then sent to the network for execution using [`send_transaction_file`].
///
/// If `force` is true, and a file exists at `transaction_params.output_path`, it will be overwritten.  If `force`
/// is false and a file exists at `transaction_params.output_path`, [`Error::FileAlreadyExists`] is returned
/// and the file will not be written.
pub fn make_transaction(
    builder_params: TransactionBuilderParams,
    transaction_params: TransactionStrParams<'_>,
    force: bool,
) -> Result<(), CliError> {
    let output = parse::output_kind(transaction_params.output_path, force);
    let transaction = create_transaction(builder_params, transaction_params, true)?;
    crate::output_transaction(output, &transaction).map_err(CliError::from)
}

/// Creates a [`Transaction`] and sends it to the network for execution.
///
/// `rpc_id_str` is the RPC ID to use for this request.
/// `node_address` is the address of the node to send the request to.
/// `verbosity_level` is the level of verbosity to use when outputting the response.
pub async fn put_transaction(
    rpc_id_str: &str,
    node_address: &str,
    verbosity_level: u64,
    builder_params: TransactionBuilderParams<'_>,
    transaction_params: TransactionStrParams<'_>,
) -> Result<SuccessResponse<PutTransactionResult>, CliError> {
    let rpc_id = parse::rpc_id(rpc_id_str);
    let verbosity_level = parse::verbosity(verbosity_level);
    let transaction = create_transaction(builder_params, transaction_params, false)?;
    put_transaction_rpc_handler(
        rpc_id,
        node_address,
        verbosity_level,
        Transaction::V1(transaction),
    )
    .await
    .map_err(CliError::from)
}
///
/// Reads a previously-saved [`TransactionV1`] from a file and sends it to the network for execution.
///
/// `rpc_id_str` is the RPC ID to use for this request. node_address is the address of the node to send the request to.
/// verbosity_level is the level of verbosity to use when outputting the response.
/// the input path is the path to the file containing the transaction to send.
pub async fn send_transaction_file(
    rpc_id_str: &str,
    node_address: &str,
    verbosity_level: u64,
    input_path: &str,
) -> Result<SuccessResponse<PutTransactionResult>, CliError> {
    let rpc_id = parse::rpc_id(rpc_id_str);
    let verbosity_level = parse::verbosity(verbosity_level);
    let transaction = read_transaction_file(input_path)?;
    put_transaction_rpc_handler(
        rpc_id,
        node_address,
        verbosity_level,
        Transaction::V1(transaction),
    )
    .await
    .map_err(CliError::from)
}

///
/// Reads a previously-saved [`TransactionV1`] from a file and sends it to the network for execution.
///
/// `rpc_id_str` is the RPC ID to use for this request. node_address is the address of the node to send the request to.
/// verbosity_level is the level of verbosity to use when outputting the response.
/// `maybe_speculative_exec_height_identifier` is the block identifier to use for this request.
///  the input path is the path to the file containing the transaction to send.
pub async fn speculative_send_transaction_file(
    rpc_id_str: &str,
    node_address: &str,
    verbosity_level: u64,
    input_path: &str,
    maybe_speculative_exec_height_identifier: &str,
) -> Result<SuccessResponse<SpeculativeExecTxnResult>, CliError> {
    let rpc_id = parse::rpc_id(rpc_id_str);
    let verbosity_level = parse::verbosity(verbosity_level);
    let transaction = read_transaction_file(input_path).unwrap();
    let maybe_speculative_exec_height_identifier =
        parse::block_identifier(maybe_speculative_exec_height_identifier)?;
    speculative_exec_txn(
        rpc_id,
        node_address,
        maybe_speculative_exec_height_identifier,
        verbosity_level,
        Transaction::V1(transaction),
    )
    .await
    .map_err(CliError::from)
}

/// Reads a previously-saved [`TransactionV1`] from a file, cryptographically signs it, and outputs it to a
/// file or stdout.
///
/// `maybe_output_path` specifies the output file path, or if empty, will print it to `stdout`.  If
/// `force` is true, and a file exists at `maybe_output_path`, it will be overwritten.  If `force`
/// is false and a file exists at `maybe_output_path`, [`Error::FileAlreadyExists`] is returned
/// and the file will not be written.
pub fn sign_transaction_file(
    input_path: &str,
    secret_key_path: &str,
    maybe_output_path: Option<&str>,
    force: bool,
) -> Result<(), CliError> {
    let output = parse::output_kind(maybe_output_path.unwrap_or(""), force);
    let secret_key = parse::secret_key_from_file(secret_key_path)?;
    crate::sign_transaction_file(input_path, &secret_key, output).map_err(CliError::from)
}

pub fn make_transaction_builder(
    transaction_builder_params: TransactionBuilderParams,
) -> Result<TransactionV1Builder, CliError> {
    match transaction_builder_params {
        TransactionBuilderParams::AddBid {
            public_key,
            delegation_rate,
            amount,
        } => {
            let transaction_builder =
                TransactionV1Builder::new_add_bid(public_key, delegation_rate, amount)?;
            Ok(transaction_builder)
        }
        TransactionBuilderParams::Delegate {
            delegator,
            validator,
            amount,
        } => {
            let transaction_builder =
                TransactionV1Builder::new_delegate(delegator, validator, amount)?;
            Ok(transaction_builder)
        }
        TransactionBuilderParams::Undelegate {
            delegator,
            validator,
            amount,
        } => {
            let transaction_builder =
                TransactionV1Builder::new_undelegate(delegator, validator, amount)?;
            Ok(transaction_builder)
        }
        TransactionBuilderParams::Redelegate {
            delegator,
            validator,
            amount,
            new_validator,
        } => {
            let transaction_builder =
                TransactionV1Builder::new_redelegate(delegator, validator, amount, new_validator)?;
            Ok(transaction_builder)
        }
        TransactionBuilderParams::InvocableEntity {
            entity_hash,
            entry_point,
        } => {
            let transaction_builder =
                TransactionV1Builder::new_targeting_invocable_entity(entity_hash, entry_point);
            Ok(transaction_builder)
        }
        TransactionBuilderParams::InvocableEntityAlias {
            entity_alias,
            entry_point,
        } => {
            let transaction_builder =
                TransactionV1Builder::new_targeting_invocable_entity_via_alias(
                    entity_alias,
                    entry_point,
                );
            Ok(transaction_builder)
        }
        TransactionBuilderParams::Package {
            package_hash,
            maybe_entity_version,
            entry_point,
        } => {
            let transaction_builder = TransactionV1Builder::new_targeting_package(
                package_hash,
                maybe_entity_version,
                entry_point,
            );
            Ok(transaction_builder)
        }
        TransactionBuilderParams::PackageAlias {
            package_alias,
            maybe_entity_version,
            entry_point,
        } => {
            let transaction_builder = TransactionV1Builder::new_targeting_package_via_alias(
                package_alias,
                maybe_entity_version,
                entry_point,
            );
            Ok(transaction_builder)
        }
        TransactionBuilderParams::Session {
            transaction_bytes,
            entry_point,
        } => {
            let transaction_builder = TransactionV1Builder::new_session(
                TransactionSessionKind::Standard,
                transaction_bytes,
                entry_point,
            );
            Ok(transaction_builder)
        }
        TransactionBuilderParams::Transfer {
            maybe_source,
            target,
            amount,
            maybe_id,
        } => {
            let transaction_builder =
                TransactionV1Builder::new_transfer(amount, maybe_source, target, maybe_id)?;

            Ok(transaction_builder)
        }
        TransactionBuilderParams::WithdrawBid { public_key, amount } => {
            let transaction_builder = TransactionV1Builder::new_withdraw_bid(public_key, amount)?;
            Ok(transaction_builder)
        }
    }
}
