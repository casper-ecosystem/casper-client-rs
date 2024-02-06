use crate::cli::{parse, CliError, TransactionBuilderParams, TransactionStrParams};
use casper_types::{InitiatorAddr, TransactionSessionKind, TransactionV1, TransactionV1Builder};

pub fn create_transaction(
    builder_params: TransactionBuilderParams,
    transaction_params: TransactionStrParams,
    allow_unsigned_deploy: bool,
) -> Result<TransactionV1, CliError> {
    let chain_name = transaction_params.chain_name.to_string();
    if transaction_params.payment_amount.is_empty() {
        return Err(CliError::InvalidArgument {
            context: "create_transaction (payment_amount)",
            error: "payment_amount is required".to_string(),
        });
    }
    let payment_amount = transaction_params
        .payment_amount
        .parse::<u64>()
        .map_err(|error| CliError::FailedToParseInt {
            context: "payment_amount",
            error,
        })?;

    let maybe_secret_key = if allow_unsigned_deploy && transaction_params.secret_key.is_empty() {
        None
    } else if transaction_params.secret_key.is_empty() && !allow_unsigned_deploy {
        return Err(CliError::InvalidArgument {
            context: "create_transaction (secret_key, allow_unsigned_deploy)",
            error: format!(
                "allow_unsigned_deploy was {}, but no secret key was provided",
                allow_unsigned_deploy
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
        .with_payment_amount(payment_amount)
        .with_timestamp(timestamp)
        .with_ttl(ttl)
        .with_chain_name(chain_name);

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
    if let Some(pricing_mode) = transaction_params.maybe_pricing_mode {
        let pricing_mode = parse::pricing_mode(pricing_mode)?;
        transaction_builder = transaction_builder.with_pricing_mode(pricing_mode);
    }
    if let Some(account) = maybe_session_account {
        transaction_builder =
            transaction_builder.with_initiator_addr(InitiatorAddr::PublicKey(account));
    }

    let txn = transaction_builder.build().map_err(crate::Error::from)?;
    Ok(txn)
}

/// Creates a transfer [`Transaction`] and outputs it to a file or stdout.
///
/// As a file, the `Transaction` can subsequently be signed by other parties using [`sign_transaction_file`]
/// and then sent to the network for execution using [`send_transaction_file`].
///
/// `maybe_output_path` specifies the output file path, or if empty, will print it to `stdout`.  If
/// `force` is true, and a file exists at `maybe_output_path`, it will be overwritten.  If `force`
/// is false and a file exists at `maybe_output_path`, [`Error::FileAlreadyExists`] is returned
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
            entity_addr,
            entry_point,
        } => {
            let transaction_builder =
                TransactionV1Builder::new_targeting_invocable_entity(entity_addr, entry_point);
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
            package_addr,
            maybe_entity_version,
            entry_point,
        } => {
            let transaction_builder = TransactionV1Builder::new_targeting_package(
                package_addr,
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
            source_uref,
            target_uref,
            amount,
            maybe_to,
            maybe_id,
        } => {
            let transaction_builder = TransactionV1Builder::new_transfer(
                source_uref,
                target_uref,
                amount,
                maybe_to,
                maybe_id,
            )?;
            Ok(transaction_builder)
        }
        TransactionBuilderParams::WithdrawBid { public_key, amount } => {
            let transaction_builder = TransactionV1Builder::new_withdraw_bid(public_key, amount)?;
            Ok(transaction_builder)
        }
    }
}
