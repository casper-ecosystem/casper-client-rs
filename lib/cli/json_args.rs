//! `CLType` and `CLValue` parsing and validation from "json args" syntax.

mod error;
pub mod help;

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use casper_types::{
    bytesrepr::{ToBytes, OPTION_NONE_TAG, OPTION_SOME_TAG, RESULT_ERR_TAG, RESULT_OK_TAG},
    AsymmetricType, CLType, CLValue, Key, NamedArg, PublicKey, URef, U128, U256, U512,
};

use crate::cli::CliError;
pub use error::{Error, ErrorDetails};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(super) struct JsonArg {
    name: String,
    #[serde(rename = "type")]
    cl_type: CLType,
    value: Value,
}

impl TryFrom<JsonArg> for NamedArg {
    type Error = CliError;

    fn try_from(json_arg: JsonArg) -> Result<Self, Self::Error> {
        let mut bytes = vec![];
        write_json_to_bytesrepr(&json_arg.cl_type, &json_arg.value, &mut bytes).map_err(
            |details| {
                Error::new(
                    json_arg.name.clone(),
                    json_arg.cl_type.clone(),
                    json_arg.value,
                    details,
                )
            },
        )?;
        Ok(NamedArg::new(
            json_arg.name,
            CLValue::from_components(json_arg.cl_type, bytes),
        ))
    }
}

fn write_json_to_bytesrepr(
    cl_type: &CLType,
    json_value: &Value,
    output: &mut Vec<u8>,
) -> Result<(), ErrorDetails> {
    match (cl_type, json_value) {
        (&CLType::Bool, Value::Bool(bool)) => bool.write_bytes(output)?,
        (&CLType::I32, Value::Number(number)) => {
            let value = number
                .as_i64()
                .and_then(|value| i32::try_from(value).ok())
                .ok_or(ErrorDetails::CannotParseToI32)?;
            value.write_bytes(output)?
        }
        (&CLType::I64, Value::Number(number)) => {
            let value = number.as_i64().ok_or(ErrorDetails::CannotParseToI64)?;
            value.write_bytes(output)?
        }
        (&CLType::U8, Value::Number(number)) => {
            let value = number
                .as_u64()
                .and_then(|value| u8::try_from(value).ok())
                .ok_or(ErrorDetails::CannotParseToU8)?;
            value.write_bytes(output)?
        }
        (&CLType::U32, Value::Number(number)) => {
            let value = number
                .as_u64()
                .and_then(|value| u32::try_from(value).ok())
                .ok_or(ErrorDetails::CannotParseToU32)?;
            value.write_bytes(output)?
        }
        (&CLType::U64, Value::Number(number)) => {
            let value = number.as_u64().ok_or(ErrorDetails::CannotParseToU64)?;
            value.write_bytes(output)?
        }
        (&CLType::U128, Value::String(string)) => {
            let value = U128::from_dec_str(string)?;
            value.write_bytes(output)?
        }
        (&CLType::U128, Value::Number(number)) => {
            let value = number.as_u64().ok_or(ErrorDetails::CannotParseToU64)?;
            U128::from(value).write_bytes(output)?
        }
        (&CLType::U256, Value::String(string)) => {
            let value = U256::from_dec_str(string)?;
            value.write_bytes(output)?
        }
        (&CLType::U256, Value::Number(number)) => {
            let value = number.as_u64().ok_or(ErrorDetails::CannotParseToU64)?;
            U256::from(value).write_bytes(output)?
        }
        (&CLType::U512, Value::String(string)) => {
            let value = U512::from_dec_str(string)?;
            value.write_bytes(output)?
        }
        (&CLType::U512, Value::Number(number)) => {
            let value = number.as_u64().ok_or(ErrorDetails::CannotParseToU64)?;
            U512::from(value).write_bytes(output)?
        }
        (&CLType::Unit, Value::Null) => (),
        (&CLType::String, Value::String(string)) => string.write_bytes(output)?,
        (&CLType::Key, Value::String(string)) => {
            let value = Key::from_formatted_str(string)?;
            value.write_bytes(output)?
        }
        (&CLType::Key, Value::Object(map)) => {
            // This is an alternative JSON representation of a `Key`, e.g. if calling
            // `serde_json::to_string()` on a `Key` enum.
            if map.len() != 1 {
                return Err(ErrorDetails::KeyObjectHasInvalidNumberOfFields);
            }
            let (mapped_variant, mapped_string) = match map.iter().next() {
                Some((k, Value::String(v))) => (k, v),
                _ => return Err(ErrorDetails::KeyObjectHasInvalidFieldType),
            };
            let value = Key::from_formatted_str(mapped_string)?;
            // Return an error if the variant name doesn't match the parsed `Key` variant.
            match value {
                Key::Account(_) if mapped_variant == "Account" => {}
                Key::Hash(_) if mapped_variant == "Hash" => {}
                Key::URef(_) if mapped_variant == "URef" => {}
                Key::Transfer(_) if mapped_variant == "Transfer" => {}
                Key::DeployInfo(_) if mapped_variant == "DeployInfo" => {}
                Key::EraInfo(_) if mapped_variant == "EraInfo" => {}
                Key::Balance(_) if mapped_variant == "Balance" => {}
                Key::Bid(_) if mapped_variant == "Bid" => {}
                Key::Withdraw(_) if mapped_variant == "Withdraw" => {}
                Key::Dictionary(_) if mapped_variant == "Dictionary" => {}
                Key::SystemEntityRegistry if mapped_variant == "SystemEntityRegistry" => {}
                Key::Unbond(_) if mapped_variant == "Unbond" => {}
                Key::ChainspecRegistry if mapped_variant == "ChainspecRegistry" => {}
                _ => return Err(ErrorDetails::KeyObjectHasInvalidVariant),
            }
            value.write_bytes(output)?
        }
        (&CLType::URef, Value::String(string)) => {
            let value = URef::from_formatted_str(string)?;
            value.write_bytes(output)?
        }
        (&CLType::PublicKey, Value::String(string)) => {
            let value = PublicKey::from_hex(string)?;
            value.write_bytes(output)?
        }
        (CLType::Option(ref _inner_cl_type), Value::Null) => {
            output.push(OPTION_NONE_TAG);
        }
        (CLType::Option(ref inner_cl_type), _) => {
            output.push(OPTION_SOME_TAG);
            write_json_to_bytesrepr(inner_cl_type, json_value, output)?
        }
        (CLType::List(ref inner_cl_type), Value::Array(vec)) => {
            (vec.len() as u32).write_bytes(output)?;
            for item in vec {
                write_json_to_bytesrepr(inner_cl_type, item, output)?;
            }
        }
        (CLType::List(ref inner_cl_type), Value::String(string)) => {
            // Support passing a `Vec<u8>` as a hex-encoded string.
            if **inner_cl_type != CLType::U8 {
                return Err(ErrorDetails::IncompatibleType);
            }
            let mut value = base16::decode(string)?;
            (value.len() as u32).write_bytes(output)?;
            output.append(&mut value);
        }
        (&CLType::ByteArray(expected_length), Value::String(string)) => {
            let mut value = base16::decode(string)?;
            let actual_length = value.len() as u32;
            if actual_length != expected_length {
                return Err(ErrorDetails::ByteArrayLengthMismatch {
                    expected_length,
                    actual_length,
                });
            }
            output.append(&mut value);
        }
        (&CLType::ByteArray(expected_length), Value::Array(array)) => {
            // While we generally output byte arrays in JSON as hex-encoded strings, we want to
            // support explicit JSON arrays too.
            let actual_length = array.len() as u32;
            if actual_length != expected_length {
                return Err(ErrorDetails::ByteArrayLengthMismatch {
                    expected_length,
                    actual_length,
                });
            }
            for value in array {
                let byte = value
                    .as_u64()
                    .and_then(|value| u8::try_from(value).ok())
                    .ok_or(ErrorDetails::CannotParseToU8)?;
                output.push(byte);
            }
        }
        (CLType::Result { ref ok, ref err }, Value::Object(map)) => {
            if map.len() != 1 {
                return Err(ErrorDetails::ResultObjectHasInvalidNumberOfFields);
            }
            match map.iter().next() {
                Some((key, value)) if key.to_ascii_lowercase() == "ok" => {
                    output.push(RESULT_OK_TAG);
                    write_json_to_bytesrepr(ok, value, output)?
                }
                Some((key, value)) if key.to_ascii_lowercase() == "err" => {
                    output.push(RESULT_ERR_TAG);
                    write_json_to_bytesrepr(err, value, output)?
                }
                _ => return Err(ErrorDetails::ResultObjectHasInvalidVariant),
            }
        }
        (
            CLType::Map {
                key: ref key_type,
                value: ref value_type,
            },
            Value::Object(map),
        ) => {
            // While we generally output maps in JSON as arrays of objects of the form
            // '[{"key":KEY-1,"value":VALUE-1},{"key":KEY-2,"value":VALUE-2}]', we want to
            // support maps which can be converted to JSON via `serde_json::to_string`.  Such maps
            // are limited to ones where the key type is a String or Number, and will take the form
            // '{"KEY-1":VALUE-1,"KEY-2":VALUE-2}'.
            match **key_type {
                CLType::I32
                | CLType::I64
                | CLType::U8
                | CLType::U32
                | CLType::U64
                | CLType::U128
                | CLType::U256
                | CLType::U512
                | CLType::String => (),
                _ => return Err(ErrorDetails::MapTypeNotValidAsObject(*key_type.clone())),
            };
            (map.len() as u32).write_bytes(output)?;
            for (key_as_str, value) in map.iter() {
                let key = match **key_type {
                    CLType::I32 => json!(i32::from_str(key_as_str)?),
                    CLType::I64 => json!(i64::from_str(key_as_str)?),
                    CLType::U8 => json!(u8::from_str(key_as_str)?),
                    CLType::U32 => json!(u32::from_str(key_as_str)?),
                    CLType::U64 => json!(u64::from_str(key_as_str)?),
                    CLType::U128 => json!(U128::from_dec_str(key_as_str)?),
                    CLType::U256 => json!(U256::from_dec_str(key_as_str)?),
                    CLType::U512 => json!(U512::from_dec_str(key_as_str)?),
                    CLType::String => json!(key_as_str),
                    _ => return Err(ErrorDetails::MapTypeNotValidAsObject(*key_type.clone())),
                };
                write_json_to_bytesrepr(key_type, &key, output)?;
                write_json_to_bytesrepr(value_type, value, output)?;
            }
        }
        (
            CLType::Map {
                key: ref key_type,
                value: ref value_type,
            },
            Value::Array(array),
        ) => {
            (array.len() as u32).write_bytes(output)?;
            for item in array {
                let map = if let Some(map) = item.as_object() {
                    map
                } else {
                    return Err(ErrorDetails::MapArrayHasInvalidEntryType);
                };
                if map.len() != 2 {
                    return Err(ErrorDetails::MapEntryObjectHasInvalidNumberOfFields);
                }
                let key = map
                    .get("key")
                    .ok_or(ErrorDetails::MapEntryObjectMissingKeyField)?;
                write_json_to_bytesrepr(key_type, key, output)?;
                let value = map
                    .get("value")
                    .ok_or(ErrorDetails::MapEntryObjectMissingValueField)?;
                write_json_to_bytesrepr(value_type, value, output)?;
            }
        }
        (CLType::Tuple1(ref inner_cl_types), Value::Array(vec)) => {
            if vec.len() != inner_cl_types.len() {
                return Err(ErrorDetails::TupleEntryCountMismatch {
                    expected: inner_cl_types.len(),
                    actual: vec.len(),
                });
            }
            write_json_to_bytesrepr(&inner_cl_types[0], &vec[0], output)?
        }
        (CLType::Tuple2(ref inner_cl_types), Value::Array(vec)) => {
            if vec.len() != inner_cl_types.len() {
                return Err(ErrorDetails::TupleEntryCountMismatch {
                    expected: inner_cl_types.len(),
                    actual: vec.len(),
                });
            }
            write_json_to_bytesrepr(&inner_cl_types[0], &vec[0], output)?;
            write_json_to_bytesrepr(&inner_cl_types[1], &vec[1], output)?
        }
        (CLType::Tuple3(ref inner_cl_types), Value::Array(vec)) => {
            if vec.len() != inner_cl_types.len() {
                return Err(ErrorDetails::TupleEntryCountMismatch {
                    expected: inner_cl_types.len(),
                    actual: vec.len(),
                });
            }
            write_json_to_bytesrepr(&inner_cl_types[0], &vec[0], output)?;
            write_json_to_bytesrepr(&inner_cl_types[1], &vec[1], output)?;
            write_json_to_bytesrepr(&inner_cl_types[2], &vec[2], output)?
        }
        _ => return Err(ErrorDetails::IncompatibleType),
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use casper_types::{account::AccountHash, bytesrepr::Bytes, AccessRights, CLTyped, EraId};

    use super::*;

    const ARRAY: [u8; 32] = [17; 32];

    fn arg_name() -> String {
        "arg name".to_string()
    }

    fn create_input(type_str: &str, value_str: &str) -> String {
        format!(
            r#"{{"name":"{}","type":{},"value":{}}}"#,
            arg_name(),
            type_str,
            value_str
        )
    }

    fn should_parse<T: ToBytes + CLTyped + Serialize>(
        type_str: &str,
        value_str: &str,
        expected: T,
    ) {
        let input = create_input(type_str, value_str);
        let parsed: JsonArg = serde_json::from_str(&input).unwrap();
        let named_arg = NamedArg::try_from(parsed)
            .unwrap_or_else(|error| panic!("unexpected error: {}", error));
        let expected = NamedArg::new(arg_name(), CLValue::from_t(expected).unwrap());
        assert_eq!(named_arg, expected);
    }

    fn get_error(type_str: &str, value_str: &str) -> ErrorDetails {
        let input = create_input(type_str, value_str);
        let json_arg: JsonArg = serde_json::from_str(&input).unwrap();
        let mut bytes = vec![];
        write_json_to_bytesrepr(&json_arg.cl_type, &json_arg.value, &mut bytes).unwrap_err()
    }

    #[test]
    fn should_parse_bool() {
        const TYPE: &str = r#""Bool""#;
        should_parse(TYPE, "true", true);
        should_parse(TYPE, "false", false);
    }

    #[test]
    fn should_parse_i32() {
        const TYPE: &str = r#""I32""#;
        should_parse(TYPE, "-2147483648", i32::MIN);
        should_parse(TYPE, "-1", -1_i32);
        should_parse(TYPE, "0", 0_i32);
        should_parse(TYPE, "1", 1_i32);
        should_parse(TYPE, "2147483647", i32::MAX);
    }

    #[test]
    fn should_parse_i64() {
        const TYPE: &str = r#""I64""#;
        should_parse(TYPE, "-9223372036854775808", i64::MIN);
        should_parse(TYPE, "-1", -1_i64);
        should_parse(TYPE, "0", 0_i64);
        should_parse(TYPE, "1", 1_i64);
        should_parse(TYPE, "9223372036854775807", i64::MAX);
    }

    #[test]
    fn should_parse_u8() {
        const TYPE: &str = r#""U8""#;
        should_parse(TYPE, "0", 0_u8);
        should_parse(TYPE, "255", u8::MAX);
    }

    #[test]
    fn should_parse_u32() {
        const TYPE: &str = r#""U32""#;
        should_parse(TYPE, "0", 0_u32);
        should_parse(TYPE, "4294967295", u32::MAX);
    }

    #[test]
    fn should_parse_u64() {
        const TYPE: &str = r#""U64""#;
        should_parse(TYPE, "0", 0_u64);
        should_parse(TYPE, "18446744073709551615", u64::MAX);
    }

    #[test]
    fn should_parse_u128() {
        const TYPE: &str = r#""U128""#;
        // From string.
        should_parse(TYPE, r#""0""#, U128::zero());
        should_parse(
            TYPE,
            r#""340282366920938463463374607431768211455""#,
            U128::MAX,
        );

        // From number.
        should_parse(TYPE, "0", U128::zero());
        should_parse(TYPE, "18446744073709551615", U128::from(u64::MAX));
    }

    #[test]
    fn should_parse_u256() {
        const TYPE: &str = r#""U256""#;
        // From string.
        should_parse(TYPE, r#""0""#, U256::zero());
        should_parse(
            TYPE,
            r#""115792089237316195423570985008687907853269984665640564039457584007913129639935""#,
            U256::MAX,
        );

        // From number.
        should_parse(TYPE, "0", U256::zero());
        should_parse(TYPE, "18446744073709551615", U256::from(u64::MAX));
    }

    #[test]
    fn should_parse_u512() {
        const TYPE: &str = r#""U512""#;
        // From string.
        should_parse(TYPE, r#""0""#, U512::zero());
        should_parse(
            TYPE,
            r#""13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095""#,
            U512::MAX,
        );

        // From number.
        should_parse(TYPE, "0", U512::zero());
        should_parse(TYPE, "18446744073709551615", U512::from(u64::MAX));
    }

    #[test]
    fn should_parse_unit() {
        const TYPE: &str = r#""Unit""#;
        should_parse(TYPE, "null", ());
    }

    #[test]
    fn should_parse_string() {
        const TYPE: &str = r#""String""#;
        should_parse(TYPE, r#""""#, String::new());
        should_parse(TYPE, r#""some text""#, "some text".to_string());
    }

    #[test]
    fn should_parse_key() {
        const TYPE: &str = r#""Key""#;
        const UREF: URef = URef::new(ARRAY, AccessRights::NONE);

        // From formatted string.
        should_parse(
            TYPE,
            r#""account-hash-1111111111111111111111111111111111111111111111111111111111111111""#,
            Key::Account(AccountHash::new(ARRAY)),
        );
        // From JSON Object.
        should_parse(
            TYPE,
            r#"{"Account":"account-hash-1111111111111111111111111111111111111111111111111111111111111111"}"#,
            Key::Account(AccountHash::new(ARRAY)),
        );

        // From formatted string.
        should_parse(
            TYPE,
            r#""hash-1111111111111111111111111111111111111111111111111111111111111111""#,
            Key::Hash(ARRAY),
        );
        // From JSON Object.
        should_parse(
            TYPE,
            r#"{"Hash":"hash-1111111111111111111111111111111111111111111111111111111111111111"}"#,
            Key::Hash(ARRAY),
        );

        // From formatted string.
        should_parse(
            TYPE,
            r#""uref-1111111111111111111111111111111111111111111111111111111111111111-000""#,
            Key::URef(UREF),
        );
        // From JSON Object.
        should_parse(
            TYPE,
            r#"{"URef":"uref-1111111111111111111111111111111111111111111111111111111111111111-000"}"#,
            Key::URef(UREF),
        );

        // From formatted string.
        should_parse(TYPE, r#""era-1""#, Key::EraInfo(EraId::new(1)));
        // From JSON Object.
        should_parse(TYPE, r#"{"EraInfo":"era-1"}"#, Key::EraInfo(EraId::new(1)));
    }

    #[test]
    fn should_parse_uref() {
        const TYPE: &str = r#""URef""#;
        should_parse(
            TYPE,
            r#""uref-1111111111111111111111111111111111111111111111111111111111111111-007""#,
            URef::new(ARRAY, AccessRights::all()),
        );
    }

    #[test]
    fn should_parse_public_key() {
        const TYPE: &str = r#""PublicKey""#;
        should_parse(
            TYPE,
            r#""011111111111111111111111111111111111111111111111111111111111111111""#,
            PublicKey::ed25519_from_bytes(ARRAY).unwrap(),
        );
    }

    #[test]
    fn should_parse_option() {
        const SIMPLE_TYPE: &str = r#"{"Option":"U64"}"#;
        should_parse(SIMPLE_TYPE, "999", Some(999_u64));
        should_parse::<Option<u64>>(SIMPLE_TYPE, "null", None);

        const COMPLEX_TYPE: &str = r#"{"Option":{"Tuple2":["Bool","U8"]}}"#;
        should_parse(COMPLEX_TYPE, "[true,1]", Some((true, 1_u8)));
        should_parse::<Option<(bool, u8)>>(COMPLEX_TYPE, "null", None);
    }

    #[test]
    fn should_parse_list() {
        const SIMPLE_TYPE: &str = r#"{"List":"U64"}"#;
        should_parse(SIMPLE_TYPE, "[1,2,3]", vec![1_u64, 2, 3]);
        should_parse(SIMPLE_TYPE, "[]", Vec::<u64>::new());

        const COMPLEX_TYPE: &str = r#"{"List":{"Option":{"Tuple2":["Bool","U8"]}}}"#;
        should_parse(
            COMPLEX_TYPE,
            "[[true,1],null,[false,2]]",
            vec![Some((true, 1_u8)), None, Some((false, 2))],
        );
        should_parse(COMPLEX_TYPE, "[]", Vec::<Option<(bool, u8)>>::new());

        // For List of U8 only, we can parse from a JSON String.
        const BYTE_LIST_TYPE: &str = r#"{"List":"U8"}"#;
        should_parse(
            BYTE_LIST_TYPE,
            r#""0102ff""#,
            Bytes::from(vec![1_u8, 2, 255]),
        );
        should_parse(BYTE_LIST_TYPE, r#""""#, Bytes::from(vec![]));
    }

    #[test]
    fn should_parse_byte_array() {
        const BYTE_ARRAY_3_TYPE: &str = r#"{"ByteArray":3}"#;
        const BYTE_ARRAY_3: [u8; 3] = [1, 20, 255];
        // From hex-encoded string.
        should_parse(BYTE_ARRAY_3_TYPE, r#""0114ff""#, BYTE_ARRAY_3);
        // From array.
        should_parse(BYTE_ARRAY_3_TYPE, "[1,20,255]", BYTE_ARRAY_3);

        const BYTE_ARRAY_EMPTY_TYPE: &str = r#"{"ByteArray":0}"#;
        // From hex-encoded string.
        should_parse(BYTE_ARRAY_EMPTY_TYPE, r#""""#, []);
        // From array.
        should_parse(BYTE_ARRAY_EMPTY_TYPE, "[]", []);
    }

    #[test]
    fn should_parse_result() {
        const SIMPLE_TYPE: &str = r#"{"Result":{"ok":"Bool","err":"U8"}}"#;

        should_parse::<Result<bool, u8>>(SIMPLE_TYPE, r#"{"Ok":true}"#, Ok(true));
        should_parse::<Result<bool, u8>>(SIMPLE_TYPE, r#"{"ok":true}"#, Ok(true));
        should_parse::<Result<bool, u8>>(SIMPLE_TYPE, r#"{"Err":1}"#, Err(1));
        should_parse::<Result<bool, u8>>(SIMPLE_TYPE, r#"{"err":1}"#, Err(1));

        const COMPLEX_TYPE: &str =
            r#"{"Result":{"ok":{"Option":{"Tuple2":["Bool","U8"]}},"err":{"Option":"String"}}}"#;
        should_parse::<Result<Option<(bool, u8)>, Option<String>>>(
            COMPLEX_TYPE,
            r#"{"Ok":[true,255]}"#,
            Ok(Some((true, 255))),
        );
        should_parse::<Result<Option<(bool, u8)>, Option<String>>>(
            COMPLEX_TYPE,
            r#"{"Err":"failed"}"#,
            Err(Some("failed".to_string())),
        );
    }

    #[test]
    fn should_parse_map() {
        const SIMPLE_TYPE: &str = r#"{"Map":{"key":"U8","value":"Bool"}}"#;
        let mut simple_map = BTreeMap::new();
        simple_map.insert(1_u8, true);
        simple_map.insert(2, false);

        // From a JSON Object, only applicable where the key type is a Number or String.
        let value_str = r#"{"1":true,"2":false}"#;
        assert_eq!(value_str, serde_json::to_string(&simple_map).unwrap());
        should_parse(SIMPLE_TYPE, value_str, simple_map.clone());
        // From a JSON Array of Objects each with a 'key' and 'value' field.
        should_parse(
            SIMPLE_TYPE,
            r#"[{"key":1,"value":true},{"key":2,"value":false}]"#,
            simple_map,
        );

        // Empty map, from a JSON Object.
        should_parse(SIMPLE_TYPE, "{}", BTreeMap::<u8, bool>::new());
        // Empty map, from a JSON Array.
        should_parse(SIMPLE_TYPE, "[]", BTreeMap::<u8, bool>::new());

        const COMPLEX_TYPE: &str =
            r#"{"Map":{"key":{"List":"U64"},"value":{"Map":{"key":"U8","value":"Bool"}}}}"#;
        let list1 = vec![1_u64, 2, 3];
        let mut simple_map1 = BTreeMap::new();
        simple_map1.insert(10_u8, true);
        simple_map1.insert(20, false);
        let list2 = vec![4_u64, 5, 6];
        let mut simple_map2 = BTreeMap::new();
        simple_map2.insert(30_u8, true);
        simple_map2.insert(40, false);
        let mut complex_map = BTreeMap::new();
        complex_map.insert(list1, simple_map1);
        complex_map.insert(list2, simple_map2);
        assert!(serde_json::to_string(&complex_map).is_err());

        // Should fail from a JSON Object, as the key type is not a Number or String.
        let input = create_input(COMPLEX_TYPE, value_str);
        let parsed: JsonArg = serde_json::from_str(&input).unwrap();
        assert!(NamedArg::try_from(parsed).is_err());

        // From a JSON Array of Objects each with a 'key' and 'value' field.
        should_parse(
            COMPLEX_TYPE,
            r#"[{"key":[1,2,3],"value":{"10":true,"20":false}},{"key":[4,5,6],"value":{"30":true,"40":false}}]"#,
            complex_map,
        );
    }

    #[test]
    fn should_parse_tuple1() {
        const SIMPLE_TYPE: &str = r#"{"Tuple1":["Bool"]}"#;
        should_parse(SIMPLE_TYPE, "[true]", (true,));

        const COMPLEX_TYPE: &str =
            r#"{"Tuple1":[{"Result":{"ok":{"Option":"String"},"err":"I32"}}]}"#;
        should_parse::<(Result<Option<String>, i32>,)>(
            COMPLEX_TYPE,
            r#"[{"Ok":null}]"#,
            (Ok(None),),
        );
    }

    #[test]
    fn should_parse_tuple2() {
        const SIMPLE_TYPE: &str = r#"{"Tuple2":["Bool","U8"]}"#;
        should_parse(SIMPLE_TYPE, "[true,128]", (true, 128_u8));

        const COMPLEX_TYPE1: &str =
            r#"{"Tuple2":[{"Result":{"ok":{"Option":"String"},"err":"I32"}},"Bool"]}"#;
        should_parse::<(Result<Option<String>, i32>, bool)>(
            COMPLEX_TYPE1,
            r#"[{"Ok":null},true]"#,
            (Ok(None), true),
        );

        const COMPLEX_TYPE2: &str =
            r#"{"Tuple2":["Bool",{"Result":{"ok":{"Option":"String"},"err":"I32"}}]}"#;
        should_parse::<(bool, Result<Option<String>, i32>)>(
            COMPLEX_TYPE2,
            r#"[true,{"Ok":null}]"#,
            (true, Ok(None)),
        );
    }

    #[test]
    fn should_parse_tuple3() {
        const SIMPLE_TYPE: &str = r#"{"Tuple3":["Bool","U8","String"]}"#;
        should_parse(
            SIMPLE_TYPE,
            r#"[true,128,"a"]"#,
            (true, 128_u8, "a".to_string()),
        );

        const COMPLEX_TYPE1: &str =
            r#"{"Tuple3":[{"Result":{"ok":{"Option":"String"},"err":"I32"}},"Bool","Unit"]}"#;
        should_parse::<(Result<Option<String>, i32>, bool, ())>(
            COMPLEX_TYPE1,
            r#"[{"Ok":null},true,null]"#,
            (Ok(None), true, ()),
        );

        const COMPLEX_TYPE2: &str =
            r#"{"Tuple3":["Bool",{"Result":{"ok":{"Option":"String"},"err":"I32"}},"Unit"]}"#;
        should_parse::<(bool, Result<Option<String>, i32>, ())>(
            COMPLEX_TYPE2,
            r#"[true,{"Ok":null},null]"#,
            (true, Ok(None), ()),
        );

        const COMPLEX_TYPE3: &str =
            r#"{"Tuple3":["Bool","Unit",{"Result":{"ok":{"Option":"String"},"err":"I32"}}]}"#;
        should_parse::<(bool, (), Result<Option<String>, i32>)>(
            COMPLEX_TYPE3,
            r#"[true,null,{"Ok":null}]"#,
            (true, (), Ok(None)),
        );
    }

    #[test]
    fn should_fail_to_parse_key_from_object() {
        const TYPE: &str = r#""Key""#;

        // Object has extra field.
        let error = get_error(TYPE, r#"{"EraInfo":"era-1","extra field":null}"#);
        assert!(
            matches!(error, ErrorDetails::KeyObjectHasInvalidNumberOfFields),
            "{}",
            error
        );

        // Object has no fields.
        let error = get_error(TYPE, "{}");
        assert!(
            matches!(error, ErrorDetails::KeyObjectHasInvalidNumberOfFields),
            "{}",
            error
        );

        // Object's key is not a valid `Key` variant.
        let error = get_error(TYPE, r#"{"not a key variant":"era-1"}"#);
        assert!(
            matches!(error, ErrorDetails::KeyObjectHasInvalidVariant),
            "{}",
            error
        );

        // Object's value is an Array, not a `Key` as a formatted string.
        let error = get_error(TYPE, r#"{"EraInfo":["era-1"]}"#);
        assert!(
            matches!(error, ErrorDetails::KeyObjectHasInvalidFieldType),
            "{}",
            error
        );

        // Object's value is a String, but not a validly-formatted Key string.
        let error = get_error(TYPE, r#"{"EraInfo":"er-1"}"#);
        assert!(
            matches!(error, ErrorDetails::ParseKeyFromString(_)),
            "{}",
            error
        );
    }

    #[test]
    fn should_fail_to_parse_byte_array() {
        const TYPE: &str = r#"{"ByteArray":3}"#;

        // Parsing from String: wrong number of hex-decoded bytes.
        let error = get_error(TYPE, r#""01020304""#);
        assert!(
            matches!(
                error,
                ErrorDetails::ByteArrayLengthMismatch {
                    expected_length: 3,
                    actual_length: 4
                }
            ),
            "{}",
            error
        );

        // Parsing from String: invalid hex-encoded string.
        let error = get_error(TYPE, r#""01020g""#);
        assert!(matches!(error, ErrorDetails::HexDecode(_)), "{}", error);

        // Parsing from Array: wrong number of bytes.
        let error = get_error(TYPE, "[1,2,3,4]");
        assert!(
            matches!(
                error,
                ErrorDetails::ByteArrayLengthMismatch {
                    expected_length: 3,
                    actual_length: 4
                }
            ),
            "{}",
            error
        );

        // Parsing from Array: invalid element.
        let error = get_error(TYPE, "[1,2,256]");
        assert!(matches!(error, ErrorDetails::CannotParseToU8), "{}", error);
    }

    #[test]
    fn should_fail_to_parse_result() {
        const TYPE: &str = r#"{"Result":{"ok":"Bool","err":"U8"}}"#;

        // Object has extra field.
        let error = get_error(TYPE, r#"{"Ok":true,"extra field":null}"#);
        assert!(
            matches!(error, ErrorDetails::ResultObjectHasInvalidNumberOfFields),
            "{}",
            error
        );

        // Object has no fields.
        let error = get_error(TYPE, "{}");
        assert!(
            matches!(error, ErrorDetails::ResultObjectHasInvalidNumberOfFields),
            "{}",
            error
        );

        // Object's key is not a valid `Result` variant.
        let error = get_error(TYPE, r#"{"A-ok":true}"#);
        assert!(
            matches!(error, ErrorDetails::ResultObjectHasInvalidVariant),
            "{}",
            error
        );
    }

    #[test]
    fn should_fail_to_parse_map_from_object() {
        const INVALID_KEY_TYPE_FOR_OBJECT: &str = r#"{"Map":{"key":"Bool","value":"U8"}}"#;

        // `CLType::Map<Bool, _>` cannot be represented as a JSON Object.
        let error = get_error(INVALID_KEY_TYPE_FOR_OBJECT, r#"{"true":1}"#);
        assert!(
            matches!(error, ErrorDetails::MapTypeNotValidAsObject(CLType::Bool)),
            "{}",
            error
        );

        // As above, but with an empty Object should still fail.
        let error = get_error(INVALID_KEY_TYPE_FOR_OBJECT, "{}");
        assert!(
            matches!(error, ErrorDetails::MapTypeNotValidAsObject(CLType::Bool)),
            "{}",
            error
        );
    }

    #[test]
    fn should_fail_to_parse_map_from_array() {
        const TYPE: &str = r#"{"Map":{"key":"U8","value":"Bool"}}"#;

        // From an Array, but with a non-Object entry.
        let error = get_error(TYPE, r#"[{"key":1,"value":true},"non-object entry"]"#);
        assert!(
            matches!(error, ErrorDetails::MapArrayHasInvalidEntryType),
            "{}",
            error
        );

        // From an Array, but with an entry containing an extra field.
        let error = get_error(
            TYPE,
            r#"[{"key":1,"value":true},{"key":2,"value":false,"extra field":null}]"#,
        );
        assert!(
            matches!(error, ErrorDetails::MapEntryObjectHasInvalidNumberOfFields),
            "{}",
            error
        );

        // From an Array, but with an entry missing the "key" field.
        let error = get_error(
            TYPE,
            r#"[{"key":1,"value":true},{"not key":2,"value":false}]"#,
        );
        assert!(
            matches!(error, ErrorDetails::MapEntryObjectMissingKeyField),
            "{}",
            error
        );

        // From an Array, but with an entry missing the "key" field.
        let error = get_error(
            TYPE,
            r#"[{"key":1,"value":true},{"key":2,"not value":false}]"#,
        );
        assert!(
            matches!(error, ErrorDetails::MapEntryObjectMissingValueField),
            "{}",
            error
        );
    }

    #[test]
    fn should_fail_to_parse_tuple1() {
        const TYPE: &str = r#"{"Tuple1":["Bool"]}"#;

        // From an Array with too few entries.
        let error = get_error(TYPE, "[]");
        assert!(
            matches!(
                error,
                ErrorDetails::TupleEntryCountMismatch {
                    expected: 1,
                    actual: 0
                }
            ),
            "{}",
            error
        );

        // From an Array with too many entries.
        let error = get_error(TYPE, "[true,1]");
        assert!(
            matches!(
                error,
                ErrorDetails::TupleEntryCountMismatch {
                    expected: 1,
                    actual: 2
                }
            ),
            "{}",
            error
        );
    }

    #[test]
    fn should_fail_to_parse_tuple2() {
        const TYPE: &str = r#"{"Tuple2":["Bool","U8"]}"#;

        // From an Array with too few entries.
        let error = get_error(TYPE, "[true]");
        assert!(
            matches!(
                error,
                ErrorDetails::TupleEntryCountMismatch {
                    expected: 2,
                    actual: 1
                }
            ),
            "{}",
            error
        );

        // From an Array with too many entries.
        let error = get_error(TYPE, "[true,1,null]");
        assert!(
            matches!(
                error,
                ErrorDetails::TupleEntryCountMismatch {
                    expected: 2,
                    actual: 3
                }
            ),
            "{}",
            error
        );
    }

    #[test]
    fn should_fail_to_parse_tuple3() {
        const TYPE: &str = r#"{"Tuple3":["Bool","U8","String"]}"#;

        // From an Array with too few entries.
        let error = get_error(TYPE, "[true,1]");
        assert!(
            matches!(
                error,
                ErrorDetails::TupleEntryCountMismatch {
                    expected: 3,
                    actual: 2
                }
            ),
            "{}",
            error
        );

        // From an Array with too many entries.
        let error = get_error(TYPE, r#"[true,1,"a",null]"#);
        assert!(
            matches!(
                error,
                ErrorDetails::TupleEntryCountMismatch {
                    expected: 3,
                    actual: 4
                }
            ),
            "{}",
            error
        );
    }
}
