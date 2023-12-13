use casper_types::{InitiatorAddr, TransactionSessionKind, TransactionV1, TransactionV1Builder};
use crate::cli::{CliError, parse, SessionStrParams, TransactionStrParams};
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

    let txn = transaction_builder.build().map_err(crate::Error::from)?;
    Ok(deploy)
}
