use std::{num::ParseIntError, str::ParseBoolError};

use humantime::{DurationError, TimestampError};
use thiserror::Error;

#[cfg(doc)]
use casper_types::{
    account::AccountHash, Key, NamedArg, PublicKey, RuntimeArgs, TimeDiff, Timestamp, URef,
};
use casper_types::{CLValueError, KeyFromStrError, UIntParseError, URefFromStrError};
pub use uint::FromDecStrErr;

use crate::cli::JsonArgsError;
#[cfg(doc)]
use crate::rpcs::{DictionaryItemIdentifier, GlobalStateIdentifier};

/// Error that can be returned by the `cli` API.
#[derive(Error, Debug)]
pub enum CliError {
    /// Failed to parse a [`Key`] from a formatted string.
    #[error("failed to parse {context} as a key: {error}")]
    FailedToParseKey {
        /// Contextual description of where this error occurred.
        context: &'static str,
        /// The actual error raised.
        error: KeyFromStrError,
    },

    /// Failed to parse a [`PublicKey`] from a formatted string.
    #[error("failed to parse {context} as a public key: {error}")]
    FailedToParsePublicKey {
        /// Contextual description of where this error occurred.
        context: String,
        /// The actual error raised.
        error: casper_types::crypto::Error,
    },

    /// Failed to parse an [`AccountHash`] from a formatted string.
    #[error("failed to parse {context} as an account hash: {error}")]
    FailedToParseAccountHash {
        /// Contextual description of where this error occurred.
        context: &'static str,
        /// The actual error raised.
        error: casper_types::addressable_entity::FromStrError,
    },

    /// Failed to parse an [`casper_types::AddressableEntityHash`] from a formatted string.
    #[error("failed to parse {context} as an addressable entity hash: {error}")]
    FailedToParseAddressableEntityHash {
        /// Contextual description of where this error occurred.
        context: &'static str,
        /// The actual error raised.
        error: casper_types::addressable_entity::FromStrError,
    },

    /// Failed to parse a [`URef`] from a formatted string.
    #[error("failed to parse '{context}' as a uref: {error}")]
    FailedToParseURef {
        /// Contextual description of where this error occurred including relevant paths,
        /// filenames, etc.
        context: &'static str,
        /// The actual error raised.
        error: URefFromStrError,
    },

    /// Failed to parse an integer from a string.
    #[error("failed to parse '{context}' as an integer: {error}")]
    FailedToParseInt {
        /// Contextual description of where this error occurred including relevant paths,
        /// filenames, etc.
        context: &'static str,
        /// The actual error raised.
        error: ParseIntError,
    },

    /// Failed to parse a bool from a string.
    #[error("failed to parse '{context}' as a bool: {error}")]
    FailedToParseBool {
        /// Contextual description of where this error occurred including relevant paths,
        /// filenames, etc.
        context: &'static str,
        /// The actual error raised.
        error: ParseBoolError,
    },

    /// Failed to parse an integer from a string.
    #[error("failed to parse '{context}' as an integer: {error}")]
    FailedToParseDec {
        /// Contextual description of where this error occurred including relevant paths,
        /// filenames, etc.
        context: &'static str,
        /// The actual error raised.
        error: FromDecStrErr,
    },

    /// Failed to parse a [`TimeDiff`] from a formatted string.
    #[error("failed to parse '{context}' as a time diff: {error}")]
    FailedToParseTimeDiff {
        /// Contextual description of where this error occurred including relevant paths,
        /// filenames, etc.
        context: &'static str,
        /// The actual error raised.
        error: DurationError,
    },

    /// Failed to parse a [`Timestamp`] from a formatted string.
    #[error("failed to parse '{context}' as a timestamp: {error}")]
    FailedToParseTimestamp {
        /// Contextual description of where this error occurred including relevant paths,
        /// filenames, etc.
        context: &'static str,
        /// The actual error raised.
        error: TimestampError,
    },

    /// Failed to parse a `U128`, `U256` or `U512` from a string.
    #[error("failed to parse '{context}' as u128, u256, or u512: {error:?}")]
    FailedToParseUint {
        /// Contextual description of where this error occurred including relevant paths,
        /// filenames, etc.
        context: &'static str,
        /// The actual error raised.
        error: UIntParseError,
    },

    /// Failed to parse a `Digest` from a string.
    #[error("failed to parse '{context}' as a hash digest: {error:?}")]
    FailedToParseDigest {
        /// Contextual description of where this error occurred including relevant paths,
        /// filenames, etc.
        context: &'static str,
        /// The actual error raised.
        error: casper_types::DigestError,
    },

    /// Failed to create a [`GlobalStateIdentifier`].
    #[error("failed to parse state identifier")]
    FailedToParseStateIdentifier,

    /// Conflicting arguments.
    #[error("conflicting arguments passed '{context}' {args:?}")]
    ConflictingArguments {
        /// Contextual description of where this error occurred including relevant paths,
        /// filenames, etc.
        context: String,
        /// Arguments passed, with their values.
        args: Vec<String>,
    },

    /// Invalid `CLValue`.
    #[error("invalid CLValue error: {0}")]
    InvalidCLValue(String),

    /// Invalid argument.
    #[error("invalid argument '{context}': {error}")]
    InvalidArgument {
        /// Contextual description of where this error occurred including relevant paths,
        /// filenames, etc.
        context: &'static str,
        /// An error message.
        error: String,
    },

    /// Error while parsing the json-args from a string to JSON.
    #[error(
        "failed to parse json-args to JSON: {0}.  They should be a JSON Array of Objects, each of \
        the form {{\"name\":<String>,\"type\":<VALUE>,\"value\":<VALUE>}}"
    )]
    FailedToParseJsonArgs(#[from] serde_json::Error),

    /// Error while building a [`NamedArg`] from parsed JSON input.
    #[error(transparent)]
    JsonArgs(#[from] JsonArgsError),

    /// Core error.
    #[error(transparent)]
    Core(#[from] crate::Error),

    /// Failed to parse a package address
    #[error("Failed to parse a package address")]
    FailedToParsePackageAddr,

    /// Failed to parse a transfer target
    #[error("Failed to parse a transfer target")]
    FailedToParseTransferTarget,

    /// Failed to parse a validator public key.
    #[error("Failed to parse a validator public key")]
    FailedToParseValidatorPublicKey,
}

impl From<CLValueError> for CliError {
    fn from(error: CLValueError) -> Self {
        match error {
            CLValueError::Serialization(bytesrepr_error) => CliError::Core(bytesrepr_error.into()),
            CLValueError::Type(type_mismatch) => {
                CliError::InvalidCLValue(type_mismatch.to_string())
            }
        }
    }
}
