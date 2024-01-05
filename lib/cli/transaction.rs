use crate::cli::{parse, CliError, TransactionStrParams};
use casper_types::{InitiatorAddr, TransactionV1, TransactionV1Builder};

pub fn with_payment_and_session(
    builder: TransactionV1Builder,
    transaction_params: TransactionStrParams,
    payment_amount: &str,
    allow_unsigned_deploy: bool,
) -> Result<TransactionV1, CliError> {
    let chain_name = transaction_params.chain_name.to_string();
    let payment_amount =
        payment_amount
            .parse::<u64>()
            .map_err(|error| CliError::FailedToParseInt {
                context: "with_payment_and_session",
                error,
            })?; // move this parsing to a module in parse for cleanliness here.

    let maybe_secret_key = if allow_unsigned_deploy && transaction_params.secret_key.is_empty() {
        None
    } else if transaction_params.secret_key.is_empty() && !allow_unsigned_deploy {
        return Err(CliError::InvalidArgument {
            context: "with_payment_and_session (secret_key, allow_unsigned_deploy)",
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
    let maybe_session_account = parse::session_account(transaction_params.initiator_addr)?;

    let mut transaction_builder = builder
        .with_payment_amount(payment_amount)
        .with_timestamp(timestamp)
        .with_ttl(ttl)
        .with_chain_name(chain_name);

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
    maybe_output_path: &str,
    builder: TransactionV1Builder,
    transaction_params: TransactionStrParams<'_>,
    payment_amount: &str,
    force: bool,
) -> Result<(), CliError> {
    let output = parse::output_kind(maybe_output_path, force);
    let transaction = with_payment_and_session(builder, transaction_params, payment_amount, true)?;
    crate::output_transaction(output, &transaction).map_err(CliError::from)
}
