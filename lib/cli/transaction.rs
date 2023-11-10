use crate::cli::transaction_str_params::TransactionStrParams;
use casper_types::{ExecutableDeployItem, Transaction, TransactionV1Builder, TransactionV1Kind};

use super::{parse, CliError, SessionStrParams};

/// Creates a new transaction using TransactionStrParams, and SessionStrParams
pub fn create_transaction(
    transaction_params: TransactionStrParams,
    session_params: SessionStrParams,
    allow_unsigned_transaction: bool,
) -> Result<Transaction, CliError> {
    let chain_name = transaction_params.chain_name.to_string();
    let session = parse::session_executable_deploy_item(session_params)?;
    let maybe_secret_key = if allow_unsigned_transaction && transaction_params.secret_key.is_empty()
    {
        None
    } else if transaction_params.secret_key.is_empty() && !allow_unsigned_transaction {
        return Err(CliError::InvalidArgument {
            context: "with_payment_and_session (secret_key, allow_unsigned_deploy)",
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
    let maybe_session_account = parse::session_account(transaction_params.session_account)?;

    let mut transaction_builder = TransactionV1Builder::new()
        .with_chain_name(chain_name)
        .with_timestamp(timestamp)
        .with_ttl(ttl);

    if let Ok(payment) = transaction_params.payment_amount.parse::<u64>() {
        transaction_builder = transaction_builder.with_payment(payment);
    }

    if let Some(account) = maybe_session_account {
        transaction_builder = transaction_builder.with_account(account);
    } else if let Some(secret_key) = maybe_secret_key {
        transaction_builder = transaction_builder.with_secret_key(&secret_key);
    }

    //Here we should add a helper function to properly build the body from the ExecutableDeployItem
    //for now this just returns the TransactionKind to represent a get-era-validators body
    //my plan is to use the transaction_kind and transaction_args in the TransactionStrParams
    //in conjunction with the parsed session code to grab all of the essential data for the transaction body.

    let body = create_transaction_body(
        transaction_params.transaction_kind,
        transaction_params.transaction_kind_args,
        session,
    )?;

    transaction_builder = transaction_builder.with_body(body);

    let transaction = transaction_builder.build().map_err(crate::Error::from)?;
    transaction.has_valid_hash().map_err(crate::Error::from)?;
    Ok(Transaction::V1(transaction))
}

/// This function utilizes the transaction_kind, transaction_arg_list, and session code to determine what sort of transaction to create
fn create_transaction_body(
    _transaction_kind: &str,
    _transaction_arg_list: Vec<&str>,
    _session: ExecutableDeployItem,
) -> Result<TransactionV1Kind, CliError> {
    Ok(TransactionV1Kind::new_get_era_validators())
}
