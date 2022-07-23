use std::num::ParseIntError;

use serde_json::Value;
use thiserror::Error;

#[cfg(doc)]
use casper_types::{account::AccountHash, CLValue, Key, NamedArg, PublicKey, URef};
use casper_types::{
    bytesrepr::Error as BytesreprError, crypto::Error as CryptoError, CLType, KeyFromStrError,
    URefFromStrError,
};

/// Error associated with parsing a JSON arg into a [`NamedArg`].
#[derive(Error, Debug)]
#[error(
    "failed to construct a CLValue of type {cl_type:?} from {json_value} for arg {arg_name}: \
    {details}"
)]
pub struct Error {
    /// The name of the relevant argument.
    pub arg_name: String,
    /// The `CLType` into which the value should be parsed.
    pub cl_type: CLType,
    /// The JSON value from which parsing failed.
    pub json_value: Value,
    /// Details of the error.
    pub details: ErrorDetails,
}

impl Error {
    pub(super) fn new(
        arg_name: String,
        cl_type: CLType,
        json_value: Value,
        details: ErrorDetails,
    ) -> Self {
        Error {
            arg_name,
            cl_type,
            json_value,
            details,
        }
    }
}

/// Details of an error associated with parsing a JSON arg into a [`NamedArg`].
#[derive(Error, Debug)]
pub enum ErrorDetails {
    /// bytesrepr error while constructing a [`CLValue`].
    #[error("failed bytesrepr encoding: {0}")]
    Bytesrepr(BytesreprError),

    /// Cannot convert the given JSON Number to an `i32`.
    #[error("cannot convert given JSON Number to `i32`")]
    CannotParseToI32,

    /// Cannot convert the given JSON Number to an `i64`.
    #[error("cannot convert given JSON Number to `i64`")]
    CannotParseToI64,

    /// Cannot convert the given JSON Number to a `u8`.
    #[error("cannot convert given JSON Number to `u8`")]
    CannotParseToU8,

    /// Cannot convert the given JSON Number to a `u32`.
    #[error("cannot convert given JSON Number to `u32`")]
    CannotParseToU32,

    /// Cannot convert the given JSON Number to a `u64`.
    #[error("cannot convert given JSON Number to `u64`")]
    CannotParseToU64,

    /// Error parsing an integer from a decimal string representation.
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),

    /// Error parsing a big integer from a decimal string representation.
    #[error(transparent)]
    ParseBigint(#[from] uint::FromDecStrErr),

    /// Error parsing a [`Key`] from a formatted string representation.
    #[error("failed parsing a Key: {0}")]
    ParseKeyFromString(KeyFromStrError),

    /// JSON Object representing a [`Key`] must have the structure
    /// `{"<KEY VARIANT>":"<KEY AS FORMATTED STRING>"}`, but the given one does not have just one
    /// field.
    #[error(
        "invalid number of fields: JSON Object representing a Key must have exactly one field, the \
        name of the Key variant mapped to the Key as a formatted string"
    )]
    KeyObjectHasInvalidNumberOfFields,

    /// JSON Object representing a [`Key`] must have the structure
    /// `{"<KEY VARIANT>":"<KEY AS FORMATTED STRING>"}`, but the given one does not have a String
    /// value.
    #[error(
        "invalid field type: JSON Object representing a Key must have exactly one field, the name \
        of the Key variant mapped to the Key as a formatted string"
    )]
    KeyObjectHasInvalidFieldType,

    /// JSON Object representing a [`Key`] must have the structure
    /// `{"<KEY VARIANT>":"<KEY AS FORMATTED STRING>"}`, but the given one does match any valid
    /// `Key` variant.
    #[error(
        "invalid Key variant: JSON Object representing a Key must have exactly one field, the name \
        of the Key variant mapped to the Key as a formatted string"
    )]
    KeyObjectHasInvalidVariant,

    /// Error parsing a [`URef`] from a formatted string representation.
    #[error("failed parsing a URef: {0}")]
    ParseURef(URefFromStrError),

    /// Error parsing a [`PublicKey`] from a formatted string representation.
    #[error(transparent)]
    ParsePublicKey(#[from] CryptoError),

    /// Error parsing a hex string.
    #[error(transparent)]
    HexDecode(#[from] base16::DecodeError),

    /// Number of hex-decoded bytes not as expected.
    #[error("number of hex-decoded bytes ({actual_length}) not as expected ({expected_length})")]
    ByteArrayLengthMismatch {
        /// The expected number of bytes.
        expected_length: u32,
        /// The actual number of bytes.
        actual_length: u32,
    },

    /// JSON Object representing a `Result` must have the structure `{"Ok":<VALUE>}` or
    /// `{"Err":<VALUE>}`, but the given one does not have just one field.
    #[error(
        "invalid number of fields: JSON Object representing a Result must have exactly one field \
        named 'Ok' or 'Err'"
    )]
    ResultObjectHasInvalidNumberOfFields,

    /// JSON Object representing a `Result` must have the structure `{"Ok":<VALUE>}` or
    /// `{"Err":<VALUE>}`, but the given one is neither `"Ok"` nor `"Err"`.
    #[error(
        "invalid Result variant: JSON Object representing a Result must have exactly one field \
        named 'Ok' or 'Err'"
    )]
    ResultObjectHasInvalidVariant,

    /// `CLType::Map`s can only be represented as JSON Objects if the map's key type is a string or
    /// number.
    #[error(
        "invalid map key type ({0:?}): only maps with key types of string or number can be \
        represented as JSON Objects, maps with more complex key types must use a JSON Array"
    )]
    MapTypeNotValidAsObject(CLType),

    /// JSON Array representing a `CLType::Map` must have all entries with the structure
    /// `{"key":<VALUE>,"value":<VALUE>}`, but the given one has an entry which is not an Object.
    #[error("invalid entry type: JSON Array representing a Map must have all entries as Objects")]
    MapArrayHasInvalidEntryType,

    /// JSON Object representing a map entry must have the structure
    /// `{"key":<VALUE>,"value":<VALUE>}`, but the given one does not have exactly two fields.
    #[error(
        "invalid number of fields: JSON Object representing a Map entry must have exactly two \
        fields, named 'key' and 'value'"
    )]
    MapEntryObjectHasInvalidNumberOfFields,

    /// JSON Object representing a map entry must have the structure
    /// `{"key":<VALUE>,"value":<VALUE>}`, but the given one does not have `"key"`.
    #[error(
        "missing key field: JSON Object representing a Map entry must have exactly two fields, \
        named 'key' and 'value'"
    )]
    MapEntryObjectMissingKeyField,

    /// JSON Object representing a map entry must have the structure
    /// `{"key":<VALUE>,"value":<VALUE>}`, but the given one does not have `"value"`.
    #[error(
        "missing value field: JSON Object representing a Map entry must have exactly two fields, \
        named 'key' and 'value'"
    )]
    MapEntryObjectMissingValueField,

    /// Number of tuple entries not as expected.
    #[error("number of tuple entries ({actual}) not as expected ({expected})")]
    TupleEntryCountMismatch {
        /// The expected number of tuple entries.
        expected: usize,
        /// The actual number of tuple entries.
        actual: usize,
    },

    /// The given `CLType` fundamentally cannot be constructed from the given type of JSON value.
    #[error("the given CLType cannot be constructed from the given type of JSON value")]
    IncompatibleType,
}

impl From<KeyFromStrError> for ErrorDetails {
    fn from(error: KeyFromStrError) -> Self {
        ErrorDetails::ParseKeyFromString(error)
    }
}

impl From<URefFromStrError> for ErrorDetails {
    fn from(error: URefFromStrError) -> Self {
        ErrorDetails::ParseURef(error)
    }
}

impl From<BytesreprError> for ErrorDetails {
    fn from(error: BytesreprError) -> Self {
        ErrorDetails::Bytesrepr(error)
    }
}
