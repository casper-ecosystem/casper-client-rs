use casper_types::{InitiatorAddr, TransactionSessionKind, TransactionV1, TransactionV1Builder};
use crate::cli::{CliError, parse, SessionStrParams, transaction, TransactionStrParams};
use crate::Error;

pub fn with_payment_and_session(
    transaction_params: TransactionStrParams,
    payment_amount: &str,
    session_params: SessionStrParams,
    allow_unsigned_deploy: bool,
) -> Result<TransactionV1, CliError> {
    let chain_name = transaction_params.chain_name.to_string();
    let entry_point = session_params.session_entry_point.to_string();
    let payment_amount = payment_amount.parse::<u64>().map_err( |error|
        CliError::FailedToParseInt { context: "with_payment_and_session", error }
    )?; // move this parsing to a module in parse for cleanliness here.

    // The below function should be a proper parsing function for the generic session params
    // it should have checks in place to ensure proper params have been supplied and should do more
    // than return bytes for the session
    let session = parse::temp_transaction_module_bytes(session_params.session_path)?;

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

    let mut transaction_builder = TransactionV1Builder::new_session(TransactionSessionKind::Standard, session, entry_point)
        .with_payment_amount(payment_amount.into())
        .with_timestamp(timestamp)
        .with_ttl(ttl)
        .with_chain_name(chain_name);

    if let Some(secret_key) = &maybe_secret_key {
        transaction_builder = transaction_builder.with_secret_key(secret_key);
    }
    if let Some(account) = maybe_session_account {
        transaction_builder = transaction_builder.with_initiator_addr(InitiatorAddr::PublicKey(account));
    }

    let deploy = transaction_builder.build().map_err(crate::Error::from)?;
    Ok(deploy)
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
    transaction_params: TransactionStrParams<'_>,
    session_params: SessionStrParams<'_>,
    payment_amount: &str,
    force: bool,
) -> Result<(), CliError>{
    let output = parse::output_kind(maybe_output_path, force);
    let deploy =
        transaction::with_payment_and_session(transaction_params, payment_amount, session_params, true)?;
    crate::output_transaction(output, &deploy).map_err(CliError::from)
}
