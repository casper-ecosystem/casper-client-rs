use casper_types::{
    account::AccountHash, AsymmetricType, PublicKey, SecretKey, UIntParseError, URef, U512,
};

use super::{parse, CliError, DeployStrParams, PaymentStrParams, SessionStrParams};
use crate::{
    types::{Deploy, DeployBuilder, MAX_SERIALIZED_SIZE_OF_DEPLOY},
    TransferTarget,
};

/// Creates new Deploy with specified payment and session data.
pub fn with_payment_and_session(
    deploy_params: DeployStrParams,
    payment_params: PaymentStrParams,
    session_params: SessionStrParams,
    allow_unsigned_deploy: bool,
) -> Result<Deploy, CliError> {
    let chain_name = deploy_params.chain_name.to_string();
    let session = parse::session_executable_deploy_item(session_params)?;
    let payment = parse::payment_executable_deploy_item(payment_params)?;
    let timestamp = parse::timestamp(deploy_params.timestamp)?;
    let ttl = parse::ttl(deploy_params.ttl)?;
    let maybe_session_account = parse::session_account(deploy_params.session_account)?;

    let mut deploy_builder = DeployBuilder::new(chain_name, session)
        .with_payment(payment)
        .with_timestamp(timestamp)
        .with_ttl(ttl);

    let maybe_secret_key = get_maybe_secret_key(deploy_params.secret_key, allow_unsigned_deploy)?;
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

    let mut deploy_builder =
        DeployBuilder::new_transfer(chain_name, amount, source_purse, target, maybe_transfer_id)
            .with_payment(payment)
            .with_timestamp(timestamp)
            .with_ttl(ttl);

    let maybe_secret_key = get_maybe_secret_key(deploy_params.secret_key, allow_unsigned_deploy)?;
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

fn get_maybe_secret_key(
    secret_key: &str,
    allow_unsigned_deploy: bool,
) -> Result<Option<SecretKey>, CliError> {
    if !secret_key.is_empty() {
        #[cfg(feature = "sdk")]
        {
            let secret_key: SecretKey = match SecretKey::from_pem(secret_key) {
                Ok(key) => key,
                Err(error) => {
                    return Err(CliError::Core(crate::Error::CryptoError {
                        context: "secret key",
                        error,
                    }));
                }
            };
            Ok(Some(secret_key))
        }
        #[cfg(not(feature = "sdk"))]
        {
            Ok(Some(parse::secret_key_from_file(secret_key)?))
        }
    } else if !allow_unsigned_deploy {
        Err(CliError::InvalidArgument {
            context: "with_payment_and_session (secret_key, allow_unsigned_deploy)",
            error: format!(
                "allow_unsigned_deploy was {}, but no secret key was provided",
                allow_unsigned_deploy
            ),
        })
    } else {
        Ok(None)
    }
}
