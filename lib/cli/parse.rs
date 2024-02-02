//! This module contains structs and helpers which are used by multiple subcommands related to
//! creating deploys.

use std::{fs, path::Path, str::FromStr};

use rand::Rng;

use casper_types::{
    account::AccountHash, bytesrepr::Bytes, crypto, AddressableEntityHash, AsymmetricType,
    BlockHash, DeployHash, Digest, ExecutableDeployItem, HashAddr, Key, NamedArg, PricingMode,
    PublicKey, RuntimeArgs, SecretKey, TimeDiff, Timestamp, TransactionV1, UIntParseError, URef,
    U512,
};

use super::{simple_args, CliError, PaymentStrParams, SessionStrParams};
use crate::{
    AccountIdentifier, BlockIdentifier, EntityIdentifier, GlobalStateIdentifier, JsonRpcId,
    OutputKind, PurseIdentifier, Verbosity,
};

pub(super) fn rpc_id(maybe_rpc_id: &str) -> JsonRpcId {
    if maybe_rpc_id.is_empty() {
        JsonRpcId::from(rand::thread_rng().gen::<i64>())
    } else if let Ok(i64_id) = maybe_rpc_id.parse::<i64>() {
        JsonRpcId::from(i64_id)
    } else {
        JsonRpcId::from(maybe_rpc_id.to_string())
    }
}

pub(super) fn verbosity(verbosity_level: u64) -> Verbosity {
    match verbosity_level {
        0 => Verbosity::Low,
        1 => Verbosity::Medium,
        _ => Verbosity::High,
    }
}

pub(super) fn output_kind(maybe_output_path: &str, force: bool) -> OutputKind {
    if maybe_output_path.is_empty() {
        OutputKind::Stdout
    } else {
        OutputKind::file(Path::new(maybe_output_path), force)
    }
}

pub(super) fn secret_key_from_file<P: AsRef<Path>>(
    secret_key_path: P,
) -> Result<SecretKey, CliError> {
    SecretKey::from_file(secret_key_path).map_err(|error| {
        CliError::Core(crate::Error::CryptoError {
            context: "secret key",
            error,
        })
    })
}

pub(super) fn timestamp(value: &str) -> Result<Timestamp, CliError> {
    if value.is_empty() {
        return Ok(Timestamp::now());
    }
    Timestamp::from_str(value).map_err(|error| CliError::FailedToParseTimestamp {
        context: "timestamp",
        error,
    })
}

pub(super) fn ttl(value: &str) -> Result<TimeDiff, CliError> {
    TimeDiff::from_str(value).map_err(|error| CliError::FailedToParseTimeDiff {
        context: "ttl",
        error,
    })
}

pub(super) fn session_account(value: &str) -> Result<Option<PublicKey>, CliError> {
    if value.is_empty() {
        return Ok(None);
    }

    let public_key = PublicKey::from_hex(value).map_err(|error| crate::Error::CryptoError {
        context: "session account",
        error: crypto::ErrorExt::from(error),
    })?;
    Ok(Some(public_key))
}

/// Handles providing the arg for and retrieval of simple session and payment args.
pub(crate) mod arg_simple {
    use super::*;

    pub(crate) mod session {
        use super::*;

        pub fn parse(values: &[&str]) -> Result<Option<RuntimeArgs>, CliError> {
            Ok(if values.is_empty() {
                None
            } else {
                Some(get(values)?)
            })
        }
    }

    pub(crate) mod payment {
        use super::*;

        pub fn parse(values: &[&str]) -> Result<Option<RuntimeArgs>, CliError> {
            Ok(if values.is_empty() {
                None
            } else {
                Some(get(values)?)
            })
        }
    }

    fn get(values: &[&str]) -> Result<RuntimeArgs, CliError> {
        let mut runtime_args = RuntimeArgs::new();
        for arg in values {
            simple_args::insert_arg(arg, &mut runtime_args)?;
        }
        Ok(runtime_args)
    }
}

pub(crate) mod args_json {
    use super::*;
    use crate::cli::JsonArg;

    pub mod session {
        use super::*;

        pub fn parse(json_str: &str) -> Result<Option<RuntimeArgs>, CliError> {
            get(json_str)
        }
    }

    pub mod payment {
        use super::*;

        pub fn parse(json_str: &str) -> Result<Option<RuntimeArgs>, CliError> {
            get(json_str)
        }
    }

    fn get(json_str: &str) -> Result<Option<RuntimeArgs>, CliError> {
        if json_str.is_empty() {
            return Ok(None);
        }
        let json_args: Vec<JsonArg> = serde_json::from_str(json_str)?;
        let mut named_args = Vec::with_capacity(json_args.len());
        for json_arg in json_args {
            named_args.push(NamedArg::try_from(json_arg)?);
        }
        Ok(Some(RuntimeArgs::from(named_args)))
    }
}

const STANDARD_PAYMENT_ARG_NAME: &str = "amount";
fn standard_payment(value: &str) -> Result<RuntimeArgs, CliError> {
    if value.is_empty() {
        return Err(CliError::InvalidCLValue(value.to_string()));
    }
    let arg = U512::from_dec_str(value).map_err(|err| CliError::FailedToParseUint {
        context: "amount",
        error: UIntParseError::FromDecStr(err),
    })?;
    let mut runtime_args = RuntimeArgs::new();
    runtime_args.insert(STANDARD_PAYMENT_ARG_NAME, arg)?;
    Ok(runtime_args)
}

/// Checks if conflicting arguments are provided for parsing session information.
///
/// # Arguments
///
/// * `context` - A string indicating the context in which the arguments are checked.
/// * `simple` - A vector of strings representing simple arguments.
/// * `json` - A string representing JSON-formatted arguments.
/// * `complex` - A string representing complex arguments.
///
/// # Returns
///
/// Returns a `Result` with an empty `Ok(())` variant if no conflicting arguments are found. If
/// conflicting arguments are provided, an `Err` variant with a `CliError::ConflictingArguments` is
/// returned.
///
/// # Errors
///
/// Returns an `Err` variant with a `CliError::ConflictingArguments` if conflicting arguments are
/// provided.
///
/// # Original Author
/// This function was modified from one of the same name written by Gregory Roussac for the 1.6 SDK
fn check_no_conflicting_arg_types(simple: &[&str], json: &str) -> Result<(), CliError> {
    let count = [!simple.is_empty(), !json.is_empty()]
        .iter()
        .filter(|&&x| x)
        .count();

    if count > 1 {
        return Err(CliError::ConflictingArguments {
            context: "Conflicting args (simple, and json) were provided",
            args: vec![simple.join(", "), json.to_owned()],
        });
    }
    Ok(())
}

pub(crate) fn args_from_simple_or_json(
    simple: Option<RuntimeArgs>,
    json: Option<RuntimeArgs>,
) -> RuntimeArgs {
    // We can have exactly zero or one of the two as `Some`.
    match (simple, json) {
        (Some(args), None) | (None, Some(args)) => args,
        (None, None) => RuntimeArgs::new(),
        _ => unreachable!("should not have more than one of simple, json or complex args"),
    }
}

/// Private macro for enforcing parameter validity.
/// e.g. check_exactly_one_not_empty!(
///   (field1) requires[another_field],
///   (field2) requires[another_field, yet_another_field]
///   (field3) requires[]
/// )
/// Returns an error if:
/// - More than one parameter is non-empty.
/// - Any parameter that is non-empty has requires[] requirements that are empty.
macro_rules! check_exactly_one_not_empty {
    ( context: $site:expr, $( ($x:expr) requires[$($y:expr),*] requires_empty[$($z:expr),*] ),+ $(,)? ) => {{

        let field_is_empty_map = &[$(
            (stringify!($x), $x.is_empty())
        ),+];

        let required_arguments = field_is_empty_map
            .iter()
            .filter(|(_, is_empty)| !*is_empty)
            .map(|(field, _)| field.to_string())
            .collect::<Vec<_>>();

        if required_arguments.is_empty() {
            let required_param_names = vec![$((stringify!($x))),+];
            return Err(CliError::InvalidArgument {
                context: $site,
                error: format!("Missing a required arg - exactly one of the following must be provided: {:?}", required_param_names),
            });
        }
        if required_arguments.len() == 1 {
            let name = &required_arguments[0];
            let field_requirements = &[$(
                (
                    stringify!($x),
                    $x,
                    vec![$((stringify!($y), $y)),*],
                    vec![$((stringify!($z), $z)),*],
                )
            ),+];

            // Check requires[] and requires_requires_empty[] fields
            let (_, value, requirements, required_empty) = field_requirements
                .iter()
                .find(|(field, _, _, _)| *field == name).expect("should exist");
            let required_arguments = requirements
                .iter()
                .filter(|(_, value)| !value.is_empty())
                .collect::<Vec<_>>();

            if requirements.len() != required_arguments.len() {
                let required_param_names = requirements
                    .iter()
                    .map(|(requirement_name, _)| requirement_name)
                    .collect::<Vec<_>>();
                return Err(CliError::InvalidArgument {
                    context: $site,
                    error: format!("Field {} also requires following fields to be provided: {:?}", name, required_param_names),
                });
            }

            let mut conflicting_fields = required_empty
                .iter()
                .filter(|(_, value)| !value.is_empty())
                .map(|(field, value)| format!("{}={}", field, value)).collect::<Vec<_>>();

            if !conflicting_fields.is_empty() {
                conflicting_fields.push(format!("{}={}", name, value));
                conflicting_fields.sort();
                return Err(CliError::ConflictingArguments{
                    context: $site,
                    args: conflicting_fields,
                });
            }
        } else {
            // Here we have more than one non-empty arg, so it is an error.  Collect all non-empty
            // fields and their values into a string to populate the returned Error.
            let mut non_empty_fields_with_values = [$((stringify!($x), $x)),+]
                .iter()
                .filter_map(|(field_name, field_value)| if !field_value.is_empty() {
                    Some(format!("{}={}", field_name, field_value))
                } else {
                    None
                })
                .collect::<Vec<String>>();
            non_empty_fields_with_values.sort();
            return Err(CliError::ConflictingArguments {
                context: $site,
                args: non_empty_fields_with_values,
            });
        }
    }}
}

pub(super) fn session_executable_deploy_item(
    params: SessionStrParams,
) -> Result<ExecutableDeployItem, CliError> {
    let SessionStrParams {
        session_hash,
        session_name,
        session_package_hash,
        session_package_name,
        session_path,
        ref session_args_simple,
        session_args_json,
        session_version,
        session_entry_point,
        is_session_transfer: session_transfer,
    } = params;
    // This is to make sure that we're using &str consistently in the macro call below.
    let is_session_transfer = if session_transfer { "true" } else { "" };

    check_exactly_one_not_empty!(
        context: "parse_session_info",
        (session_hash)
            requires[session_entry_point] requires_empty[session_version],
        (session_name)
            requires[session_entry_point] requires_empty[session_version],
        (session_package_hash)
            requires[session_entry_point] requires_empty[],
        (session_package_name)
            requires[session_entry_point] requires_empty[],
        (session_path)
            requires[] requires_empty[session_entry_point, session_version],
        (is_session_transfer)
            requires[] requires_empty[session_entry_point, session_version]
    );

    check_no_conflicting_arg_types(session_args_simple, session_args_json)?;

    let session_args = args_from_simple_or_json(
        arg_simple::session::parse(session_args_simple)?,
        args_json::session::parse(session_args_json)?,
    );
    if session_transfer {
        if session_args.is_empty() {
            return Err(CliError::InvalidArgument {
                context: "is_session_transfer",
                error: "requires --session-arg to be present".to_string(),
            });
        }
        return Ok(ExecutableDeployItem::Transfer { args: session_args });
    }
    let invalid_entry_point = || CliError::InvalidArgument {
        context: "session_entry_point",
        error: session_entry_point.to_string(),
    };
    if let Some(session_name) = name(session_name) {
        return Ok(ExecutableDeployItem::StoredContractByName {
            name: session_name,
            entry_point: entry_point(session_entry_point).ok_or_else(invalid_entry_point)?,
            args: session_args,
        });
    }

    if let Some(session_hash) = contract_hash(session_hash)? {
        return Ok(ExecutableDeployItem::StoredContractByHash {
            hash: session_hash.into(),
            entry_point: entry_point(session_entry_point).ok_or_else(invalid_entry_point)?,
            args: session_args,
        });
    }

    let version = version(session_version)?;
    if let Some(package_name) = name(session_package_name) {
        return Ok(ExecutableDeployItem::StoredVersionedContractByName {
            name: package_name,
            version, // defaults to highest enabled version
            entry_point: entry_point(session_entry_point).ok_or_else(invalid_entry_point)?,
            args: session_args,
        });
    }

    if let Some(package_hash) = contract_hash(session_package_hash)? {
        return Ok(ExecutableDeployItem::StoredVersionedContractByHash {
            hash: package_hash.into(),
            version, // defaults to highest enabled version
            entry_point: entry_point(session_entry_point).ok_or_else(invalid_entry_point)?,
            args: session_args,
        });
    }

    let module_bytes = fs::read(session_path).map_err(|error| crate::Error::IoError {
        context: format!("unable to read session file at '{}'", session_path),
        error,
    })?;
    Ok(ExecutableDeployItem::ModuleBytes {
        module_bytes: module_bytes.into(),
        args: session_args,
    })
}
/// Parse a transaction file into Bytes to be used in crafting a new session transaction
pub fn transaction_module_bytes(session_path: &str) -> Result<Bytes, CliError> {
    let module_bytes = fs::read(session_path).map_err(|error| crate::Error::IoError {
        context: format!("unable to read session file at '{}'", session_path),
        error,
    })?;
    Ok(Bytes::from(module_bytes))
}

/// Parse a transaction file into a `TransactionV1` to be sent to the network
pub fn transaction_from_file(transaction_path: &str) -> Result<TransactionV1, CliError> {
    let transaction_bytes = fs::read(transaction_path).map_err(|error| crate::Error::IoError {
        context: format!("unable to read transaction file at '{}'", transaction_path),
        error,
    })?;
    let transaction_str =
        std::str::from_utf8(&transaction_bytes).map_err(|error| crate::Error::Utf8Error {
            context: "transaction_from_file",
            error,
        })?;
    let transaction: TransactionV1 = serde_json::from_str(transaction_str).map_err(|error| {
        crate::Error::FailedToDecodeFromJson {
            context: "transaction",
            error,
        }
    })?;
    Ok(transaction)
}
/// Parses a URef from a formatted string for the purposes of creating transactions.
pub fn uref(uref_str: &str) -> Result<URef, CliError> {
    match URef::from_formatted_str(uref_str) {
        Ok(uref) => Ok(uref),
        Err(err) => Err(CliError::FailedToParseURef {
            context: "Failed to parse URef for transaction",
            error: err,
        }),
    }
}

pub(super) fn payment_executable_deploy_item(
    params: PaymentStrParams,
) -> Result<ExecutableDeployItem, CliError> {
    let PaymentStrParams {
        payment_amount,
        payment_hash,
        payment_name,
        payment_package_hash,
        payment_package_name,
        payment_path,
        ref payment_args_simple,
        payment_args_json,
        payment_version,
        payment_entry_point,
    } = params;
    check_exactly_one_not_empty!(
        context: "parse_payment_info",
        (payment_amount)
            requires[] requires_empty[payment_entry_point, payment_version],
        (payment_hash)
            requires[payment_entry_point] requires_empty[payment_version],
        (payment_name)
            requires[payment_entry_point] requires_empty[payment_version],
        (payment_package_hash)
            requires[payment_entry_point] requires_empty[],
        (payment_package_name)
            requires[payment_entry_point] requires_empty[],
        (payment_path) requires[] requires_empty[payment_entry_point, payment_version],
    );

    if let Ok(payment_args) = standard_payment(payment_amount) {
        return Ok(ExecutableDeployItem::ModuleBytes {
            module_bytes: vec![].into(),
            args: payment_args,
        });
    }

    let invalid_entry_point = || CliError::InvalidArgument {
        context: "payment_entry_point",
        error: payment_entry_point.to_string(),
    };

    //Check that we only have one of simple or json args, this is relevant for the SDK, but not in the context of the client.
    check_no_conflicting_arg_types(payment_args_simple, payment_args_json)?;

    let payment_args = args_from_simple_or_json(
        arg_simple::payment::parse(payment_args_simple)?,
        args_json::payment::parse(payment_args_json)?,
    );

    if let Some(payment_name) = name(payment_name) {
        return Ok(ExecutableDeployItem::StoredContractByName {
            name: payment_name,
            entry_point: entry_point(payment_entry_point).ok_or_else(invalid_entry_point)?,
            args: payment_args,
        });
    }

    if let Some(payment_hash) = contract_hash(payment_hash)? {
        return Ok(ExecutableDeployItem::StoredContractByHash {
            hash: payment_hash.into(),
            entry_point: entry_point(payment_entry_point).ok_or_else(invalid_entry_point)?,
            args: payment_args,
        });
    }

    let version = version(payment_version)?;
    if let Some(package_name) = name(payment_package_name) {
        return Ok(ExecutableDeployItem::StoredVersionedContractByName {
            name: package_name,
            version, // defaults to highest enabled version
            entry_point: entry_point(payment_entry_point).ok_or_else(invalid_entry_point)?,
            args: payment_args,
        });
    }

    if let Some(package_hash) = contract_hash(payment_package_hash)? {
        return Ok(ExecutableDeployItem::StoredVersionedContractByHash {
            hash: package_hash.into(),
            version, // defaults to highest enabled version
            entry_point: entry_point(payment_entry_point).ok_or_else(invalid_entry_point)?,
            args: payment_args,
        });
    }

    let module_bytes = fs::read(payment_path).map_err(|error| crate::Error::IoError {
        context: format!("unable to read payment file at '{}'", payment_path),
        error,
    })?;
    Ok(ExecutableDeployItem::ModuleBytes {
        module_bytes: module_bytes.into(),
        args: payment_args,
    })
}

fn contract_hash(value: &str) -> Result<Option<HashAddr>, CliError> {
    if value.is_empty() {
        return Ok(None);
    }
    match Digest::from_hex(value) {
        Ok(digest) => Ok(Some(digest.value())),
        Err(error) => {
            if let Ok(Key::Hash(hash)) = Key::from_formatted_str(value) {
                Ok(Some(hash))
            } else {
                Err(CliError::FailedToParseDigest {
                    context: "contract hash",
                    error,
                })
            }
        }
    }
}

fn name(value: &str) -> Option<String> {
    if value.is_empty() {
        return None;
    }
    Some(value.to_string())
}

fn entry_point(value: &str) -> Option<String> {
    if value.is_empty() {
        return None;
    }
    Some(value.to_string())
}

fn version(value: &str) -> Result<Option<u32>, CliError> {
    if value.is_empty() {
        return Ok(None);
    }
    let parsed = value
        .parse::<u32>()
        .map_err(|error| CliError::FailedToParseInt {
            context: "version",
            error,
        })?;
    Ok(Some(parsed))
}

pub(super) fn transfer_id(value: &str) -> Result<u64, CliError> {
    value.parse().map_err(|error| CliError::FailedToParseInt {
        context: "transfer-id",
        error,
    })
}

pub(super) fn block_identifier(
    maybe_block_identifier: &str,
) -> Result<Option<BlockIdentifier>, CliError> {
    if maybe_block_identifier.is_empty() {
        return Ok(None);
    }

    if maybe_block_identifier.len() == (Digest::LENGTH * 2) {
        let hash = Digest::from_hex(maybe_block_identifier).map_err(|error| {
            CliError::FailedToParseDigest {
                context: "block_identifier",
                error,
            }
        })?;
        Ok(Some(BlockIdentifier::Hash(BlockHash::new(hash))))
    } else {
        let height =
            maybe_block_identifier
                .parse()
                .map_err(|error| CliError::FailedToParseInt {
                    context: "block_identifier",
                    error,
                })?;
        Ok(Some(BlockIdentifier::Height(height)))
    }
}

pub(super) fn deploy_hash(deploy_hash: &str) -> Result<DeployHash, CliError> {
    let hash = Digest::from_hex(deploy_hash).map_err(|error| CliError::FailedToParseDigest {
        context: "deploy hash",
        error,
    })?;
    Ok(DeployHash::new(hash))
}

pub(super) fn key_for_query(key: &str) -> Result<Key, CliError> {
    match Key::from_formatted_str(key) {
        Ok(key) => Ok(key),
        Err(error) => {
            if let Ok(public_key) = PublicKey::from_hex(key) {
                Ok(Key::Account(public_key.to_account_hash()))
            } else {
                Err(CliError::FailedToParseKey {
                    context: "key for query",
                    error,
                })
            }
        }
    }
}

/// `maybe_block_id` can be either a block hash or a block height.
pub(super) fn global_state_identifier(
    maybe_block_id: &str,
    maybe_state_root_hash: &str,
) -> Result<Option<GlobalStateIdentifier>, CliError> {
    match block_identifier(maybe_block_id)? {
        Some(BlockIdentifier::Hash(hash)) => {
            return Ok(Some(GlobalStateIdentifier::BlockHash(hash)))
        }
        Some(BlockIdentifier::Height(height)) => {
            return Ok(Some(GlobalStateIdentifier::BlockHeight(height)))
        }
        None => (),
    }

    if maybe_state_root_hash.is_empty() {
        return Ok(None);
    }

    let state_root_hash =
        Digest::from_hex(maybe_state_root_hash).map_err(|error| CliError::FailedToParseDigest {
            context: "state root hash in global_state_identifier",
            error,
        })?;
    Ok(Some(GlobalStateIdentifier::StateRootHash(state_root_hash)))
}

/// `purse_id` can be a formatted public key, account hash, or URef.  It may not be empty.
pub(super) fn purse_identifier(purse_id: &str) -> Result<PurseIdentifier, CliError> {
    const ACCOUNT_HASH_PREFIX: &str = "account-hash-";
    const UREF_PREFIX: &str = "uref-";

    if purse_id.is_empty() {
        return Err(CliError::InvalidArgument {
            context: "purse_identifier",
            error: "cannot be empty string".to_string(),
        });
    }

    if purse_id.starts_with(ACCOUNT_HASH_PREFIX) {
        let account_hash = AccountHash::from_formatted_str(purse_id).map_err(|error| {
            CliError::FailedToParseAccountHash {
                context: "purse_identifier",
                error,
            }
        })?;
        return Ok(PurseIdentifier::MainPurseUnderAccountHash(account_hash));
    }

    if purse_id.starts_with(UREF_PREFIX) {
        let uref =
            URef::from_formatted_str(purse_id).map_err(|error| CliError::FailedToParseURef {
                context: "purse_identifier",
                error,
            })?;
        return Ok(PurseIdentifier::PurseUref(uref));
    }

    let public_key =
        PublicKey::from_hex(purse_id).map_err(|error| CliError::FailedToParsePublicKey {
            context: "purse_identifier".to_string(),
            error,
        })?;
    Ok(PurseIdentifier::MainPurseUnderPublicKey(public_key))
}

/// `account_identifier` can be a formatted public key, in the form of a hex-formatted string,
/// a pem file, or a file containing a hex formatted string, or a formatted string representing
/// an account hash.  It may not be empty.
pub(super) fn account_identifier(account_identifier: &str) -> Result<AccountIdentifier, CliError> {
    const ACCOUNT_HASH_PREFIX: &str = "account-hash-";

    if account_identifier.is_empty() {
        return Err(CliError::InvalidArgument {
            context: "account_identifier",
            error: "cannot be empty string".to_string(),
        });
    }

    if account_identifier.starts_with(ACCOUNT_HASH_PREFIX) {
        let account_hash =
            AccountHash::from_formatted_str(account_identifier).map_err(|error| {
                CliError::FailedToParseAccountHash {
                    context: "account_identifier",
                    error,
                }
            })?;
        return Ok(AccountIdentifier::AccountHash(account_hash));
    }

    let public_key = PublicKey::from_hex(account_identifier).map_err(|error| {
        CliError::FailedToParsePublicKey {
            context: "account_identifier".to_string(),
            error,
        }
    })?;
    Ok(AccountIdentifier::PublicKey(public_key))
}

/// `entity_identifier` can be a formatted public key, in the form of a hex-formatted string,
/// a pem file, or a file containing a hex formatted string, or a formatted string representing
/// an account hash.  It may not be empty.
pub(super) fn entity_identifier(entity_identifier: &str) -> Result<EntityIdentifier, CliError> {
    const ACCOUNT_ENTITY_PREFIX: &str = "account-";
    const CONTRACT_ENTITY_PREFIX: &str = "contract-";
    const ACCOUNT_HASH_PREFIX: &str = "account-hash-";

    if entity_identifier.is_empty() {
        return Err(CliError::InvalidArgument {
            context: "entity_identifier",
            error: "cannot be empty string".to_string(),
        });
    }

    if entity_identifier.starts_with(ACCOUNT_HASH_PREFIX) {
        let account_hash = AccountHash::from_formatted_str(entity_identifier).map_err(|error| {
            CliError::FailedToParseAccountHash {
                context: "entity_identifier",
                error,
            }
        })?;
        return Ok(EntityIdentifier::AccountHash(account_hash));
    }

    if let Some(suffix) = entity_identifier.strip_prefix(ACCOUNT_ENTITY_PREFIX) {
        let entity_hash = AddressableEntityHash::from_formatted_str(suffix).map_err(|error| {
            CliError::FailedToParseAddressableEntityHash {
                context: "entity_identifier",
                error,
            }
        })?;
        return Ok(EntityIdentifier::EntityHashForAccount(entity_hash));
    }

    if let Some(suffix) = entity_identifier.strip_prefix(CONTRACT_ENTITY_PREFIX) {
        let entity_hash = AddressableEntityHash::from_formatted_str(suffix).map_err(|error| {
            CliError::FailedToParseAddressableEntityHash {
                context: "entity_identifier",
                error,
            }
        })?;
        return Ok(EntityIdentifier::EntityHashForContract(entity_hash));
    }

    let public_key = PublicKey::from_hex(entity_identifier).map_err(|error| {
        CliError::FailedToParsePublicKey {
            context: "entity_identifier".to_string(),
            error,
        }
    })?;
    Ok(EntityIdentifier::PublicKey(public_key))
}

pub(super) fn pricing_mode(pricing_mode_str: &str) -> Result<PricingMode, CliError> {
    match pricing_mode_str.to_lowercase().as_str() {
        "fixed" => Ok(PricingMode::Fixed),
        "reserved" => Ok(PricingMode::Reserved),
        _ => {
            if let Ok(number) = pricing_mode_str.trim().parse() {
                Ok(PricingMode::GasPriceMultiplier(number))
            } else {
                Err(CliError::InvalidArgument {
                    context: "pricing_mode",
                    error: format!("Invalid pricing mode: {}", pricing_mode_str),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::*;

    const HASH: &str = "09dcee4b212cfd53642ab323fbef07dafafc6f945a80a00147f62910a915c4e6";
    const NAME: &str = "name";
    const PACKAGE_HASH: &str = "09dcee4b212cfd53642ab323fbef07dafafc6f945a80a00147f62910a915c4e6";
    const PACKAGE_NAME: &str = "package_name";
    const PATH: &str = "./session.wasm";
    const ENTRY_POINT: &str = "entrypoint";
    const VERSION: &str = "3";
    const TRANSFER: bool = true;

    impl<'a> TryFrom<SessionStrParams<'a>> for ExecutableDeployItem {
        type Error = CliError;

        fn try_from(params: SessionStrParams<'a>) -> Result<ExecutableDeployItem, Self::Error> {
            session_executable_deploy_item(params)
        }
    }

    impl<'a> TryFrom<PaymentStrParams<'a>> for ExecutableDeployItem {
        type Error = CliError;

        fn try_from(params: PaymentStrParams<'a>) -> Result<ExecutableDeployItem, Self::Error> {
            payment_executable_deploy_item(params)
        }
    }

    #[test]
    fn should_fail_to_parse_conflicting_session_parameters() {
        assert!(matches!(
            session_executable_deploy_item(SessionStrParams {
                session_hash: HASH,
                session_name: NAME,
                session_package_hash: PACKAGE_HASH,
                session_package_name: PACKAGE_NAME,
                session_path: PATH,
                session_args_simple: vec![],
                session_args_json: "",
                session_version: "",
                session_entry_point: "",
                is_session_transfer: false
            }),
            Err(CliError::ConflictingArguments {
                context: "parse_session_info",
                ..
            })
        ));
    }

    #[test]
    fn should_fail_to_parse_conflicting_payment_parameters() {
        assert!(matches!(
            payment_executable_deploy_item(PaymentStrParams {
                payment_amount: "12345",
                payment_hash: HASH,
                payment_name: NAME,
                payment_package_hash: PACKAGE_HASH,
                payment_package_name: PACKAGE_NAME,
                payment_path: PATH,
                payment_args_simple: vec![],
                payment_args_json: "",
                payment_version: "",
                payment_entry_point: "",
            }),
            Err(CliError::ConflictingArguments {
                context: "parse_payment_info",
                ..
            })
        ));
    }

    mod missing_args {
        use super::*;

        #[test]
        fn session_name_should_fail_to_parse_missing_entry_point() {
            let result = session_executable_deploy_item(SessionStrParams {
                session_name: NAME,
                ..Default::default()
            });

            assert!(matches!(
                result,
                Err(CliError::InvalidArgument {
                    context: "parse_session_info",
                    ..
                })
            ));
        }

        #[test]
        fn session_hash_should_fail_to_parse_missing_entry_point() {
            let result = session_executable_deploy_item(SessionStrParams {
                session_hash: HASH,
                ..Default::default()
            });

            assert!(matches!(
                result,
                Err(CliError::InvalidArgument {
                    context: "parse_session_info",
                    ..
                })
            ));
        }

        #[test]
        fn session_package_hash_should_fail_to_parse_missing_entry_point() {
            let result = session_executable_deploy_item(SessionStrParams {
                session_package_hash: PACKAGE_HASH,
                ..Default::default()
            });

            assert!(matches!(
                result,
                Err(CliError::InvalidArgument {
                    context: "parse_session_info",
                    ..
                })
            ));
        }

        #[test]
        fn session_package_name_should_fail_to_parse_missing_entry_point() {
            let result = session_executable_deploy_item(SessionStrParams {
                session_package_name: PACKAGE_NAME,
                ..Default::default()
            });

            assert!(matches!(
                result,
                Err(CliError::InvalidArgument {
                    context: "parse_session_info",
                    ..
                })
            ));
        }

        #[test]
        fn payment_name_should_fail_to_parse_missing_entry_point() {
            let result = payment_executable_deploy_item(PaymentStrParams {
                payment_name: NAME,
                ..Default::default()
            });

            assert!(matches!(
                result,
                Err(CliError::InvalidArgument {
                    context: "parse_payment_info",
                    ..
                })
            ));
        }

        #[test]
        fn payment_hash_should_fail_to_parse_missing_entry_point() {
            let result = payment_executable_deploy_item(PaymentStrParams {
                payment_hash: HASH,
                ..Default::default()
            });

            assert!(matches!(
                result,
                Err(CliError::InvalidArgument {
                    context: "parse_payment_info",
                    ..
                })
            ));
        }

        #[test]
        fn payment_package_hash_should_fail_to_parse_missing_entry_point() {
            let result = payment_executable_deploy_item(PaymentStrParams {
                payment_package_hash: PACKAGE_HASH,
                ..Default::default()
            });

            assert!(matches!(
                result,
                Err(CliError::InvalidArgument {
                    context: "parse_payment_info",
                    ..
                })
            ));
        }

        #[test]
        fn payment_package_name_should_fail_to_parse_missing_entry_point() {
            let result = payment_executable_deploy_item(PaymentStrParams {
                payment_package_name: PACKAGE_NAME,
                ..Default::default()
            });

            assert!(matches!(
                result,
                Err(CliError::InvalidArgument {
                    context: "parse_payment_info",
                    ..
                })
            ));
        }
    }

    mod conflicting_args {
        use super::*;

        /// impl_test_matrix - implements many tests for SessionStrParams or PaymentStrParams which
        /// ensures that an error is returned when the permutation they define is executed.
        ///
        /// For instance, it is neccesary to check that when `session_path` is set, other arguments
        /// are not.
        ///
        /// For example, a sample invocation with one test:
        /// ```
        /// impl_test_matrix![
        ///     type: SessionStrParams,
        ///     context: "parse_session_info",
        ///     session_str_params[
        ///         test[
        ///             session_path => PATH,
        ///             conflict: session_package_hash => PACKAGE_HASH,
        ///             requires[],
        ///             path_conflicts_with_package_hash
        ///         ]
        ///     ]
        /// ];
        /// ```
        /// This generates the following test module (with the fn name passed), with one test per
        /// line in `session_str_params[]`:
        /// ```
        /// #[cfg(test)]
        /// mod session_str_params {
        ///     use super::*;
        ///
        ///     #[test]
        ///     fn path_conflicts_with_package_hash() {
        ///         let info: StdResult<ExecutableDeployItem, _> = SessionStrParams {
        ///                 session_path: PATH,
        ///                 session_package_hash: PACKAGE_HASH,
        ///                 ..Default::default()
        ///             }
        ///             .try_into();
        ///         let mut conflicting = vec![
        ///             format!("{}={}", "session_path", PATH),
        ///             format!("{}={}", "session_package_hash", PACKAGE_HASH),
        ///         ];
        ///         conflicting.sort();
        ///         assert!(matches!(
        ///             info,
        ///             Err(CliError::ConflictingArguments {
        ///                 context: "parse_session_info",
        ///                 args: conflicting
        ///             }
        ///             ))
        ///         );
        ///     }
        /// }
        /// ```
        macro_rules! impl_test_matrix {
            (
                /// Struct for which to define the following tests. In our case, SessionStrParams or PaymentStrParams.
                type: $t:ident,
                /// Expected `context` field to be returned in the `CliError::ConflictingArguments{ context, .. }` field.
                context: $context:expr,

                /// $module will be our module name.
                $module:ident [$(
                    // many tests can be defined
                    test[
                        /// The argument's ident to be tested, followed by it's value.
                        $arg:tt => $arg_value:expr,
                        /// The conflicting argument's ident to be tested, followed by it's value.
                        conflict: $con:tt => $con_value:expr,
                        /// A list of any additional fields required by the argument, and their values.
                        requires[$($req:tt => $req_value:expr),*],
                        /// fn name for the defined test.
                        $test_fn_name:ident
                    ]
                )+]
            ) => {
                #[cfg(test)]
                mod $module {
                    use super::*;

                    $(
                        #[test]
                        fn $test_fn_name() {
                            let info: Result<ExecutableDeployItem, _> = $t {
                                $arg: $arg_value,
                                $con: $con_value,
                                $($req: $req_value,),*
                                ..Default::default()
                            }
                            .try_into();
                            let mut conflicting = vec![
                                format!("{}={}", stringify!($arg), $arg_value),
                                format!("{}={}", stringify!($con), $con_value),
                            ];
                            conflicting.sort();
                            assert!(matches!(
                                info,
                                Err(CliError::ConflictingArguments {
                                    context: $context,
                                    ..
                                }
                                ))
                            );
                        }
                    )+
                }
            };
        }

        // NOTE: there's no need to test a conflicting argument in both directions, since they
        // amount to passing two fields to a structs constructor.
        // Where a reverse test like this is omitted, a comment should be left.
        impl_test_matrix![
            type: SessionStrParams,
            context: "parse_session_info",
            session_str_params[

                // path
                test[session_path => PATH, conflict: session_package_hash => PACKAGE_HASH, requires[], path_conflicts_with_package_hash]
                test[session_path => PATH, conflict: session_package_name => PACKAGE_NAME, requires[], path_conflicts_with_package_name]
                test[session_path => PATH, conflict: session_hash =>         HASH,         requires[], path_conflicts_with_hash]
                test[session_path => PATH, conflict: session_name =>         HASH,         requires[], path_conflicts_with_name]
                test[session_path => PATH, conflict: session_version =>      VERSION,      requires[], path_conflicts_with_version]
                test[session_path => PATH, conflict: session_entry_point =>  ENTRY_POINT,  requires[], path_conflicts_with_entry_point]
                test[session_path => PATH, conflict: is_session_transfer =>  TRANSFER,     requires[], path_conflicts_with_transfer]

                // name
                test[session_name => NAME, conflict: session_package_hash => PACKAGE_HASH, requires[session_entry_point => ENTRY_POINT], name_conflicts_with_package_hash]
                test[session_name => NAME, conflict: session_package_name => PACKAGE_NAME, requires[session_entry_point => ENTRY_POINT], name_conflicts_with_package_name]
                test[session_name => NAME, conflict: session_hash =>         HASH,         requires[session_entry_point => ENTRY_POINT], name_conflicts_with_hash]
                test[session_name => NAME, conflict: session_version =>      VERSION,      requires[session_entry_point => ENTRY_POINT], name_conflicts_with_version]
                test[session_name => NAME, conflict: is_session_transfer =>  TRANSFER,     requires[session_entry_point => ENTRY_POINT], name_conflicts_with_transfer]

                // hash
                test[session_hash => HASH, conflict: session_package_hash => PACKAGE_HASH, requires[session_entry_point => ENTRY_POINT], hash_conflicts_with_package_hash]
                test[session_hash => HASH, conflict: session_package_name => PACKAGE_NAME, requires[session_entry_point => ENTRY_POINT], hash_conflicts_with_package_name]
                test[session_hash => HASH, conflict: session_version =>      VERSION,      requires[session_entry_point => ENTRY_POINT], hash_conflicts_with_version]
                test[session_hash => HASH, conflict: is_session_transfer =>  TRANSFER,     requires[session_entry_point => ENTRY_POINT], hash_conflicts_with_transfer]
                // name <-> hash is already checked
                // name <-> path is already checked

                // package_name
                // package_name + session_version is optional and allowed
                test[session_package_name => PACKAGE_NAME, conflict: session_package_hash => PACKAGE_HASH, requires[session_entry_point => ENTRY_POINT], package_name_conflicts_with_package_hash]
                test[session_package_name => VERSION, conflict: is_session_transfer => TRANSFER, requires[session_entry_point => ENTRY_POINT], package_name_conflicts_with_transfer]
                // package_name <-> hash is already checked
                // package_name <-> name is already checked
                // package_name <-> path is already checked

                // package_hash
                // package_hash + session_version is optional and allowed
                test[session_package_hash => PACKAGE_HASH, conflict: is_session_transfer => TRANSFER, requires[session_entry_point => ENTRY_POINT], package_hash_conflicts_with_transfer]
                // package_hash <-> package_name is already checked
                // package_hash <-> hash is already checked
                // package_hash <-> name is already checked
                // package_hash <-> path is already checked

            ]
        ];

        impl_test_matrix![
            type: PaymentStrParams,
            context: "parse_payment_info",
            payment_str_params[

                // amount
                test[payment_amount => PATH, conflict: payment_package_hash => PACKAGE_HASH, requires[], amount_conflicts_with_package_hash]
                test[payment_amount => PATH, conflict: payment_package_name => PACKAGE_NAME, requires[], amount_conflicts_with_package_name]
                test[payment_amount => PATH, conflict: payment_hash =>         HASH,         requires[], amount_conflicts_with_hash]
                test[payment_amount => PATH, conflict: payment_name =>         HASH,         requires[], amount_conflicts_with_name]
                test[payment_amount => PATH, conflict: payment_version =>      VERSION,      requires[], amount_conflicts_with_version]
                test[payment_amount => PATH, conflict: payment_entry_point =>  ENTRY_POINT,  requires[], amount_conflicts_with_entry_point]

                // path
                // amount <-> path is already checked
                test[payment_path => PATH, conflict: payment_package_hash => PACKAGE_HASH, requires[], path_conflicts_with_package_hash]
                test[payment_path => PATH, conflict: payment_package_name => PACKAGE_NAME, requires[], path_conflicts_with_package_name]
                test[payment_path => PATH, conflict: payment_hash =>         HASH,         requires[], path_conflicts_with_hash]
                test[payment_path => PATH, conflict: payment_name =>         HASH,         requires[], path_conflicts_with_name]
                test[payment_path => PATH, conflict: payment_version =>      VERSION,      requires[], path_conflicts_with_version]
                test[payment_path => PATH, conflict: payment_entry_point =>  ENTRY_POINT,  requires[], path_conflicts_with_entry_point]

                // name
                // amount <-> path is already checked
                test[payment_name => NAME, conflict: payment_package_hash => PACKAGE_HASH, requires[payment_entry_point => ENTRY_POINT], name_conflicts_with_package_hash]
                test[payment_name => NAME, conflict: payment_package_name => PACKAGE_NAME, requires[payment_entry_point => ENTRY_POINT], name_conflicts_with_package_name]
                test[payment_name => NAME, conflict: payment_hash =>         HASH,         requires[payment_entry_point => ENTRY_POINT], name_conflicts_with_hash]
                test[payment_name => NAME, conflict: payment_version =>      VERSION,      requires[payment_entry_point => ENTRY_POINT], name_conflicts_with_version]

                // hash
                // amount <-> hash is already checked
                test[payment_hash => HASH, conflict: payment_package_hash => PACKAGE_HASH, requires[payment_entry_point => ENTRY_POINT], hash_conflicts_with_package_hash]
                test[payment_hash => HASH, conflict: payment_package_name => PACKAGE_NAME, requires[payment_entry_point => ENTRY_POINT], hash_conflicts_with_package_name]
                test[payment_hash => HASH, conflict: payment_version =>      VERSION,      requires[payment_entry_point => ENTRY_POINT], hash_conflicts_with_version]
                // name <-> hash is already checked
                // name <-> path is already checked

                // package_name
                // amount <-> package_name is already checked
                test[payment_package_name => PACKAGE_NAME, conflict: payment_package_hash => PACKAGE_HASH, requires[payment_entry_point => ENTRY_POINT], package_name_conflicts_with_package_hash]
                // package_name <-> hash is already checked
                // package_name <-> name is already checked
                // package_name <-> path is already checked

                // package_hash
                // package_hash + session_version is optional and allowed
                // amount <-> package_hash is already checked
                // package_hash <-> package_name is already checked
                // package_hash <-> hash is already checked
                // package_hash <-> name is already checked
                // package_hash <-> path is already checked
            ]
        ];
    }

    mod param_tests {
        use super::*;

        const HASH: &str = "09dcee4b212cfd53642ab323fbef07dafafc6f945a80a00147f62910a915c4e6";
        const NAME: &str = "name";
        const PKG_NAME: &str = "pkg_name";
        const PKG_HASH: &str = "09dcee4b212cfd53642ab323fbef07dafafc6f945a80a00147f62910a915c4e6";
        const ENTRYPOINT: &str = "entrypoint";
        const VERSION: &str = "4";

        fn args_simple() -> Vec<&'static str> {
            vec!["name_01:bool='false'", "name_02:u32='42'"]
        }

        /// Sample data creation methods for PaymentStrParams
        mod session_params {
            use std::collections::BTreeMap;

            use casper_types::CLValue;

            use super::*;

            #[test]
            pub fn with_hash() {
                let params: Result<ExecutableDeployItem, CliError> =
                    SessionStrParams::with_hash(HASH, ENTRYPOINT, args_simple(), "").try_into();
                match params {
                    Ok(item @ ExecutableDeployItem::StoredContractByHash { .. }) => {
                        let actual: BTreeMap<String, CLValue> = item.args().clone().into();
                        let mut expected = BTreeMap::new();
                        expected.insert("name_01".to_owned(), CLValue::from_t(false).unwrap());
                        expected.insert("name_02".to_owned(), CLValue::from_t(42u32).unwrap());
                        assert_eq!(actual, expected);
                    }
                    other => panic!("incorrect type parsed {:?}", other),
                }
            }

            #[test]
            pub fn with_name() {
                let params: Result<ExecutableDeployItem, CliError> =
                    SessionStrParams::with_name(NAME, ENTRYPOINT, args_simple(), "").try_into();
                match params {
                    Ok(item @ ExecutableDeployItem::StoredContractByName { .. }) => {
                        let actual: BTreeMap<String, CLValue> = item.args().clone().into();
                        let mut expected = BTreeMap::new();
                        expected.insert("name_01".to_owned(), CLValue::from_t(false).unwrap());
                        expected.insert("name_02".to_owned(), CLValue::from_t(42u32).unwrap());
                        assert_eq!(actual, expected);
                    }
                    other => panic!("incorrect type parsed {:?}", other),
                }
            }

            #[test]
            pub fn with_package_name() {
                let params: Result<ExecutableDeployItem, CliError> =
                    SessionStrParams::with_package_name(
                        PKG_NAME,
                        VERSION,
                        ENTRYPOINT,
                        args_simple(),
                        "",
                    )
                    .try_into();
                match params {
                    Ok(item @ ExecutableDeployItem::StoredVersionedContractByName { .. }) => {
                        let actual: BTreeMap<String, CLValue> = item.args().clone().into();
                        let mut expected = BTreeMap::new();
                        expected.insert("name_01".to_owned(), CLValue::from_t(false).unwrap());
                        expected.insert("name_02".to_owned(), CLValue::from_t(42u32).unwrap());
                        assert_eq!(actual, expected);
                    }
                    other => panic!("incorrect type parsed {:?}", other),
                }
            }

            #[test]
            pub fn with_package_hash() {
                let params: Result<ExecutableDeployItem, CliError> =
                    SessionStrParams::with_package_hash(
                        PKG_HASH,
                        VERSION,
                        ENTRYPOINT,
                        args_simple(),
                        "",
                    )
                    .try_into();
                match params {
                    Ok(item @ ExecutableDeployItem::StoredVersionedContractByHash { .. }) => {
                        let actual: BTreeMap<String, CLValue> = item.args().clone().into();
                        let mut expected = BTreeMap::new();
                        expected.insert("name_01".to_owned(), CLValue::from_t(false).unwrap());
                        expected.insert("name_02".to_owned(), CLValue::from_t(42u32).unwrap());
                        assert_eq!(actual, expected);
                    }
                    other => panic!("incorrect type parsed {:?}", other),
                }
            }
        }
        /// Sample data creation methods for PaymentStrParams
        mod payment_params {
            use std::collections::BTreeMap;

            use casper_types::{CLValue, U512};

            use super::*;

            #[test]
            pub fn with_amount() {
                let params: Result<ExecutableDeployItem, CliError> =
                    PaymentStrParams::with_amount("100").try_into();
                match params {
                    Ok(item @ ExecutableDeployItem::ModuleBytes { .. }) => {
                        let amount = CLValue::from_t(U512::from(100)).unwrap();
                        assert_eq!(item.args().get("amount"), Some(&amount));
                    }
                    other => panic!("incorrect type parsed {:?}", other),
                }
            }

            #[test]
            pub fn with_hash() {
                let params: Result<ExecutableDeployItem, CliError> =
                    PaymentStrParams::with_hash(HASH, ENTRYPOINT, args_simple(), "").try_into();
                match params {
                    Ok(item @ ExecutableDeployItem::StoredContractByHash { .. }) => {
                        let actual: BTreeMap<String, CLValue> = item.args().clone().into();
                        let mut expected = BTreeMap::new();
                        expected.insert("name_01".to_owned(), CLValue::from_t(false).unwrap());
                        expected.insert("name_02".to_owned(), CLValue::from_t(42u32).unwrap());
                        assert_eq!(actual, expected);
                    }
                    other => panic!("incorrect type parsed {:?}", other),
                }
            }

            #[test]
            pub fn with_name() {
                let params: Result<ExecutableDeployItem, CliError> =
                    PaymentStrParams::with_name(NAME, ENTRYPOINT, args_simple(), "").try_into();
                match params {
                    Ok(item @ ExecutableDeployItem::StoredContractByName { .. }) => {
                        let actual: BTreeMap<String, CLValue> = item.args().clone().into();
                        let mut expected = BTreeMap::new();
                        expected.insert("name_01".to_owned(), CLValue::from_t(false).unwrap());
                        expected.insert("name_02".to_owned(), CLValue::from_t(42u32).unwrap());
                        assert_eq!(actual, expected);
                    }
                    other => panic!("incorrect type parsed {:?}", other),
                }
            }

            #[test]
            pub fn with_package_name() {
                let params: Result<ExecutableDeployItem, CliError> =
                    PaymentStrParams::with_package_name(
                        PKG_NAME,
                        VERSION,
                        ENTRYPOINT,
                        args_simple(),
                        "",
                    )
                    .try_into();
                match params {
                    Ok(item @ ExecutableDeployItem::StoredVersionedContractByName { .. }) => {
                        let actual: BTreeMap<String, CLValue> = item.args().clone().into();
                        let mut expected = BTreeMap::new();
                        expected.insert("name_01".to_owned(), CLValue::from_t(false).unwrap());
                        expected.insert("name_02".to_owned(), CLValue::from_t(42u32).unwrap());
                        assert_eq!(actual, expected);
                    }
                    other => panic!("incorrect type parsed {:?}", other),
                }
            }

            #[test]
            pub fn with_package_hash() {
                let params: Result<ExecutableDeployItem, CliError> =
                    PaymentStrParams::with_package_hash(
                        PKG_HASH,
                        VERSION,
                        ENTRYPOINT,
                        args_simple(),
                        "",
                    )
                    .try_into();
                match params {
                    Ok(item @ ExecutableDeployItem::StoredVersionedContractByHash { .. }) => {
                        let actual: BTreeMap<String, CLValue> = item.args().clone().into();
                        let mut expected = BTreeMap::new();
                        expected.insert("name_01".to_owned(), CLValue::from_t(false).unwrap());
                        expected.insert("name_02".to_owned(), CLValue::from_t(42u32).unwrap());
                        assert_eq!(actual, expected);
                    }
                    other => panic!("incorrect type parsed {:?}", other),
                }
            }
        }
    }

    mod account_identifier {
        use super::*;

        #[test]
        pub fn should_parse_valid_account_hash() {
            let account_hash =
                "account-hash-c029c14904b870e64c1d443d428c606740e82f341bea0f8542ca6494cef1383e";
            let parsed = account_identifier(account_hash).unwrap();
            let expected = AccountHash::from_formatted_str(account_hash).unwrap();
            assert_eq!(parsed, AccountIdentifier::AccountHash(expected));
        }

        #[test]
        pub fn should_parse_valid_public_key() {
            let public_key = "01567f0f205e83291312cd82988d66143d376cee7de904dd2605d3f4bbb69b3c80";
            let parsed = account_identifier(public_key).unwrap();
            let expected = PublicKey::from_hex(public_key).unwrap();
            assert_eq!(parsed, AccountIdentifier::PublicKey(expected));
        }

        #[test]
        pub fn should_fail_to_parse_invalid_account_hash() {
            //This is the account hash from above with several characters removed
            let account_hash =
                "account-hash-c029c14904b870e1d443d428c606740e82f341bea0f8542ca6494cef1383e";
            let parsed = account_identifier(account_hash);
            assert!(parsed.is_err());
        }

        #[test]
        pub fn should_fail_to_parse_invalid_public_key() {
            //This is the public key from above with several characters removed
            let public_key = "01567f0f205e83291312cd82988d66143d376cee7de904dd26054bbb69b3c80";
            let parsed = account_identifier(public_key);
            assert!(parsed.is_err());
        }
    }

    mod entity_identifier {
        use super::*;

        #[test]
        pub fn should_parse_valid_contract_entity_hash() {
            let entity_hash =
                "contract-addressable-entity-c029c14904b870e64c1d443d428c606740e82f341bea0f8542ca6494cef1383e";
            let parsed = entity_identifier(entity_hash).unwrap();
            let expected = AddressableEntityHash::from_formatted_str("addressable-entity-c029c14904b870e64c1d443d428c606740e82f341bea0f8542ca6494cef1383e").unwrap();
            assert_eq!(parsed, EntityIdentifier::EntityHashForContract(expected));
        }

        #[test]
        pub fn should_parse_valid_account_entity_hash() {
            let entity_hash =
                "account-addressable-entity-c029c14904b870e64c1d443d428c606740e82f341bea0f8542ca6494cef1383e";
            let parsed = entity_identifier(entity_hash).unwrap();
            let expected = AddressableEntityHash::from_formatted_str("addressable-entity-c029c14904b870e64c1d443d428c606740e82f341bea0f8542ca6494cef1383e").unwrap();
            assert_eq!(parsed, EntityIdentifier::EntityHashForAccount(expected));
        }

        #[test]
        pub fn should_parse_valid_public_key() {
            let public_key = "01567f0f205e83291312cd82988d66143d376cee7de904dd2605d3f4bbb69b3c80";
            let parsed = entity_identifier(public_key).unwrap();
            let expected = PublicKey::from_hex(public_key).unwrap();
            assert_eq!(parsed, EntityIdentifier::PublicKey(expected));
        }

        #[test]
        pub fn should_fail_to_parse_invalid_entity_hash() {
            //This is the account hash from above with several characters removed
            let entity_hash =
                "contract-addressable-entity-c029c14904b870e64c1d443d428c606740e82f341bea0f8542ca6494cef138";
            let parsed = entity_identifier(entity_hash);
            assert!(parsed.is_err());
        }

        #[test]
        pub fn should_fail_to_parse_invalid_public_key() {
            //This is the public key from above with several characters removed
            let public_key = "01567f0f205e83291312cd82988d66143d376cee7de904dd26054bbb69b3c80";
            let parsed = entity_identifier(public_key);
            assert!(parsed.is_err());
        }
    }

    mod pricing_mode {
        use super::*;
        #[test]
        fn should_parse_fixed_pricing_mode() {
            let pricing_mode_str = "fixed";
            let parsed = pricing_mode(pricing_mode_str).unwrap();
            assert_eq!(parsed, PricingMode::Fixed);
        }
        #[test]
        fn should_parse_reserved_pricing_mode() {
            let pricing_mode_str = "reserved";
            let parsed = pricing_mode(pricing_mode_str).unwrap();
            assert_eq!(parsed, PricingMode::Reserved);
        }
        #[test]
        fn should_parse_gas_price_multiplier_pricing_mode() {
            let pricing_mode_str = "10";
            let parsed = pricing_mode(pricing_mode_str).unwrap();
            assert_eq!(parsed, PricingMode::GasPriceMultiplier(10));
        }
        #[test]
        fn should_fail_to_parse_invalid_pricing_mode() {
            let pricing_mode_str = "invalid";
            let parsed = pricing_mode(pricing_mode_str);
            assert!(parsed.is_err());
            assert!(matches!(
                parsed,
                Err(CliError::InvalidArgument {
                    context: "pricing_mode",
                    ..
                })
            ));
        }
    }
}
