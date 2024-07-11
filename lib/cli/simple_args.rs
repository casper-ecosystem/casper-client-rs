//! `CLValue` parsing and validation from "simple args" syntax.

pub mod help;

use std::{fmt::Debug, str::FromStr};

use num_traits::Num;

use casper_types::{
    account::AccountHash,
    bytesrepr::{Bytes, ToBytes, OPTION_NONE_TAG, OPTION_SOME_TAG},
    AsymmetricType, CLType, CLTyped, CLValue, Key, PublicKey, RuntimeArgs, URef, U128, U256, U512,
};

use super::CliError;

type Parser = fn(&str, OptionalStatus, &str) -> Result<CLValue, CliError>;

const PREFIX_FOR_OPTION: &str = "opt_";
const BYTE_ARRAY_PREFIX: &str = "byte_array_";
const SUPPORTED_TYPES: [(&str, Parser); 17] = [
    ("bool", parse_bool),
    ("i32", parse_int::<i32>),
    ("i64", parse_int::<i64>),
    ("u8", parse_int::<u8>),
    ("u32", parse_int::<u32>),
    ("u64", parse_int::<u64>),
    ("u128", parse_int::<U128>),
    ("u256", parse_int::<U256>),
    ("u512", parse_int::<U512>),
    ("unit", parse_unit),
    ("string", parse_string),
    ("key", parse_key),
    ("account_hash", parse_account_hash),
    ("uref", parse_uref),
    ("public_key", parse_public_key),
    ("byte_list", parse_byte_list),
    ("byte_array_<NUM>", parse_byte_array),
];

#[derive(Debug, PartialEq, Eq)]
enum OptionalStatus {
    Some,
    None,
    NotOptional,
}

/// Parses to a given `CLValue` taking into account whether the arg represents an optional type or
/// not.
fn parse_cl_value<T, F>(optional_status: OptionalStatus, parse: F) -> Result<CLValue, CliError>
where
    T: CLTyped + ToBytes,
    F: FnOnce() -> Result<T, CliError>,
{
    match optional_status {
        OptionalStatus::Some => CLValue::from_t(Some(parse()?)),
        OptionalStatus::None => CLValue::from_t::<Option<T>>(None),
        OptionalStatus::NotOptional => CLValue::from_t(parse()?),
    }
    .map_err(|error| {
        CliError::InvalidCLValue(format!(
            "unable to parse cl value {:?} with optional_status {:?}",
            error, optional_status
        ))
    })
}

fn parse_bool(
    _simple_type: &str,
    optional_status: OptionalStatus,
    value: &str,
) -> Result<CLValue, CliError> {
    let parse = || match value.to_lowercase().as_str() {
        "true" | "t" => Ok(true),
        "false" | "f" => Ok(false),
        invalid => Err(CliError::InvalidCLValue(format!(
            "can't parse '{}' as a bool (should be 'true' or 'false')",
            invalid
        ))),
    };
    parse_cl_value(optional_status, parse)
}

fn parse_int<T: CLTyped + ToBytes + Debug + Num>(
    _simple_type: &str,
    optional_status: OptionalStatus,
    value: &str,
) -> Result<CLValue, CliError> {
    let parse = || {
        let bit_width = size_of::<T>() * 8;
        if value.is_empty() {
            return Err(CliError::InvalidCLValue(format!(
                "can't parse '' as u{}",
                bit_width,
            )));
        }
        T::from_str_radix(value, 10).map_err(|_| {
            CliError::InvalidCLValue(format!("can't parse '{}' as u{}", value, bit_width,))
        })
    };
    parse_cl_value(optional_status, parse)
}

fn parse_unit(
    _simple_type: &str,
    optional_status: OptionalStatus,
    value: &str,
) -> Result<CLValue, CliError> {
    let parse = || {
        if !value.is_empty() {
            return Err(CliError::InvalidCLValue(format!(
                "can't parse '{}' as unit (should be '')",
                value
            )));
        }
        Ok(())
    };
    parse_cl_value(optional_status, parse)
}

fn parse_string(
    _simple_type: &str,
    optional_status: OptionalStatus,
    value: &str,
) -> Result<CLValue, CliError> {
    let parse = || Ok(value.to_string());
    parse_cl_value(optional_status, parse)
}

fn parse_key(
    _simple_type: &str,
    optional_status: OptionalStatus,
    value: &str,
) -> Result<CLValue, CliError> {
    let parse = || {
        Key::from_formatted_str(value).map_err(|error| {
            CliError::InvalidCLValue(format!("can't parse '{}' as Key: {}", value, error))
        })
    };
    parse_cl_value(optional_status, parse)
}

fn parse_account_hash(
    _simple_type: &str,
    optional_status: OptionalStatus,
    value: &str,
) -> Result<CLValue, CliError> {
    let parse = || {
        AccountHash::from_formatted_str(value).map_err(|error| {
            CliError::InvalidCLValue(format!("can't parse '{}' as AccountHash: {}", value, error))
        })
    };
    parse_cl_value(optional_status, parse)
}

fn parse_uref(
    _simple_type: &str,
    optional_status: OptionalStatus,
    value: &str,
) -> Result<CLValue, CliError> {
    let parse = || {
        URef::from_formatted_str(value).map_err(|error| {
            CliError::InvalidCLValue(format!("can't parse '{}' as URef: {}", value, error))
        })
    };
    parse_cl_value(optional_status, parse)
}

fn parse_public_key(
    _simple_type: &str,
    optional_status: OptionalStatus,
    value: &str,
) -> Result<CLValue, CliError> {
    let parse = || {
        let pub_key = PublicKey::from_hex(value).map_err(|error| {
            CliError::InvalidCLValue(format!("can't parse '{}' as PublicKey: {}", value, error))
        })?;
        Ok(pub_key)
    };
    parse_cl_value(optional_status, parse)
}

fn parse_byte_list(
    _simple_type: &str,
    optional_status: OptionalStatus,
    value: &str,
) -> Result<CLValue, CliError> {
    let parse = || {
        base16::decode(value).map(Bytes::from).map_err(|error| {
            CliError::InvalidCLValue(format!("can't parse '{}' as a byte_list: {}", value, error))
        })
    };
    parse_cl_value(optional_status, parse)
}

fn parse_byte_array(
    simple_type: &str,
    optional_status: OptionalStatus,
    value: &str,
) -> Result<CLValue, CliError> {
    // Safe to unwrap as we already matched on `t.starts_with(BYTE_ARRAY_PREFIX)` to get here.
    let array_len_str = simple_type
        .strip_prefix("byte_array_")
        .expect("should have 'byte_array_' prefix");
    let array_len = u32::from_str(array_len_str).map_err(|_| {
        CliError::InvalidCLValue(format!(
            "can't parse '{}' of '{}' as an integer",
            array_len_str, simple_type,
        ))
    })?;

    let is_some = match optional_status {
        OptionalStatus::Some => true,
        OptionalStatus::None => {
            return Ok(CLValue::from_components(
                CLType::Option(Box::new(CLType::ByteArray(array_len))),
                vec![OPTION_NONE_TAG],
            ))
        }
        OptionalStatus::NotOptional => false,
    };

    let mut bytes = base16::decode(value).map_err(|error| {
        CliError::InvalidCLValue(format!(
            "can't parse '{}' as a byte_array: {}",
            value, error
        ))
    })?;
    if bytes.len() != array_len as usize {
        return Err(CliError::InvalidCLValue(format!(
            "provided {} bytes but specified a byte_array of {} bytes",
            bytes.len(),
            array_len
        )));
    }

    let cl_value = if is_some {
        let mut all_bytes = vec![OPTION_SOME_TAG];
        all_bytes.append(&mut bytes);
        CLValue::from_components(
            CLType::Option(Box::new(CLType::ByteArray(array_len))),
            all_bytes,
        )
    } else {
        CLValue::from_components(CLType::ByteArray(array_len), bytes)
    };
    Ok(cl_value)
}

/// Takes the input type and value and returns a tuple containing:
///   * the simple type (i.e. the type with any "opt_" prefix removed)
///   * the `OptionalStatus`: if there was an "opt_" prefix, then `None` or `Some` depending on
///     whether the value is "null" or not, or `NotOptional` if there was no "opt_" prefix
///   * the value, trimmed of leading and trailing single quotes
fn get_simple_type_and_optional_status(
    maybe_opt_type: &str,
    value: &str,
) -> Result<(String, OptionalStatus, String), CliError> {
    let maybe_opt_type = maybe_opt_type.to_lowercase();
    let (simple_type, optional_status, trimmed_value) =
        match maybe_opt_type.strip_prefix(PREFIX_FOR_OPTION) {
            Some(simple_type) => {
                if value.to_lowercase() == "null" {
                    (simple_type.to_string(), OptionalStatus::None, String::new())
                } else {
                    (
                        simple_type.to_string(),
                        OptionalStatus::Some,
                        value.trim_matches('\'').to_string(),
                    )
                }
            }
            None => (
                maybe_opt_type,
                OptionalStatus::NotOptional,
                value.trim_matches('\'').to_string(),
            ),
        };

    if value == trimmed_value {
        return Err(CliError::InvalidCLValue(format!(
            "value in simple arg should be surrounded by single quotes unless it's a null \
            optional value (value passed: {})",
            value
        )));
    }

    Ok((simple_type, optional_status, trimmed_value))
}

/// Returns a value built from a single arg which has been split into its constituent parts.
fn parts_to_cl_value(
    simple_type: &str,
    optional_status: OptionalStatus,
    trimmed_value: &str,
) -> Result<CLValue, CliError> {
    let parser = match simple_type {
        t if t == SUPPORTED_TYPES[0].0 => SUPPORTED_TYPES[0].1,
        t if t == SUPPORTED_TYPES[1].0 => SUPPORTED_TYPES[1].1,
        t if t == SUPPORTED_TYPES[2].0 => SUPPORTED_TYPES[2].1,
        t if t == SUPPORTED_TYPES[3].0 => SUPPORTED_TYPES[3].1,
        t if t == SUPPORTED_TYPES[4].0 => SUPPORTED_TYPES[4].1,
        t if t == SUPPORTED_TYPES[5].0 => SUPPORTED_TYPES[5].1,
        t if t == SUPPORTED_TYPES[6].0 => SUPPORTED_TYPES[6].1,
        t if t == SUPPORTED_TYPES[7].0 => SUPPORTED_TYPES[7].1,
        t if t == SUPPORTED_TYPES[8].0 => SUPPORTED_TYPES[8].1,
        t if t == SUPPORTED_TYPES[9].0 => SUPPORTED_TYPES[9].1,
        t if t == SUPPORTED_TYPES[10].0 => SUPPORTED_TYPES[10].1,
        t if t == SUPPORTED_TYPES[11].0 => SUPPORTED_TYPES[11].1,
        t if t == SUPPORTED_TYPES[12].0 => SUPPORTED_TYPES[12].1,
        t if t == SUPPORTED_TYPES[13].0 => SUPPORTED_TYPES[13].1,
        t if t == SUPPORTED_TYPES[14].0 => SUPPORTED_TYPES[14].1,
        t if t == SUPPORTED_TYPES[15].0 => SUPPORTED_TYPES[15].1,
        t if t.starts_with(BYTE_ARRAY_PREFIX) => SUPPORTED_TYPES[16].1,
        _ => {
            let original_type = match optional_status {
                OptionalStatus::Some | OptionalStatus::None => {
                    PREFIX_FOR_OPTION.to_string() + simple_type
                }
                OptionalStatus::NotOptional => simple_type.to_string(),
            };
            return Err(CliError::InvalidCLValue(format!(
                "unknown variant {}, expected one of {}",
                original_type,
                help::supported_cl_type_list()
            )));
        }
    };
    parser(simple_type, optional_status, trimmed_value)
}

/// Splits a single arg of the form `NAME:TYPE='VALUE'` into its constituent parts.
fn split_arg(arg: &str) -> Result<(&str, &str, &str), CliError> {
    const ARG_VALUE_NAME: &str = r#""NAME:TYPE='VALUE'" OR "NAME:TYPE=null""#;

    let parts: Vec<_> = arg.splitn(3, &[':', '='][..]).collect();
    if parts.len() != 3 {
        return Err(CliError::InvalidCLValue(format!(
            "arg {} should be formatted as {}",
            arg, ARG_VALUE_NAME
        )));
    }
    Ok((parts[0], parts[1], parts[2]))
}

/// Insert a value built from a single arg into `runtime_args`.
pub fn insert_arg(arg: &str, runtime_args: &mut RuntimeArgs) -> Result<(), CliError> {
    let (name, initial_type, value) = split_arg(arg)?;
    let (simple_type, optional_status, trimmed_value) =
        get_simple_type_and_optional_status(initial_type, value)?;
    let cl_value = parts_to_cl_value(&simple_type, optional_status, &trimmed_value)?;
    runtime_args.insert_cl_value(name, cl_value);
    Ok(())
}

#[cfg(test)]
mod tests {
    use casper_types::{
        account::AccountHash, bytesrepr::ToBytes, AccessRights, CLTyped, CLValue, NamedArg,
        PublicKey, RuntimeArgs, URef, U128, U256, U512,
    };

    use super::*;

    fn check_insert_valid_arg<T: CLTyped + ToBytes>(cli_string: &str, expected: T) {
        let expected = RuntimeArgs::from(vec![NamedArg::new(
            "x".to_string(),
            CLValue::from_t(expected).unwrap(),
        )]);

        let mut actual = RuntimeArgs::new();
        insert_arg(cli_string, &mut actual).expect("should parse");
        assert_eq!(actual, expected);
    }

    fn check_insert_invalid_arg(cli_string: &str) {
        let mut runtime_args = RuntimeArgs::new();
        let result = insert_arg(cli_string, &mut runtime_args);
        assert!(result.is_err(), "{} should be an error", cli_string);
    }

    #[test]
    fn should_insert_valid_bool_arg() {
        check_insert_valid_arg("x:bool='f'", false);
        check_insert_valid_arg("x:bool='F'", false);
        check_insert_valid_arg("x:bool='false'", false);
        check_insert_valid_arg("x:Bool='False'", false);
        check_insert_valid_arg("x:BOOL='False'", false);
        check_insert_valid_arg("x:bool='t'", true);
        check_insert_valid_arg("x:bool='T'", true);
        check_insert_valid_arg("x:bool='true'", true);
        check_insert_valid_arg("x:Bool='True'", true);
        check_insert_valid_arg("x:BOOL='TRUE'", true);
        check_insert_valid_arg("x:opt_bool='f'", Some(false));
        check_insert_valid_arg("x:Opt_Bool='t'", Some(true));
        check_insert_valid_arg::<Option<bool>>("x:OPT_BOOL=null", None);
    }

    #[test]
    fn should_fail_to_insert_invalid_bool_arg() {
        check_insert_invalid_arg("x:bool='fa'");
        check_insert_invalid_arg("x:opt_bool=''");
    }

    #[test]
    fn should_insert_valid_i32_arg() {
        check_insert_valid_arg("x:i32='2147483647'", i32::MAX);
        check_insert_valid_arg("x:I32='0'", 0_i32);
        check_insert_valid_arg("x:i32='-2147483648'", i32::MIN);
        check_insert_valid_arg("x:opt_i32='-1'", Some(-1_i32));
        check_insert_valid_arg::<Option<i32>>("x:OPT_I32=Null", None);
    }

    #[test]
    fn should_fail_to_insert_invalid_i32_arg() {
        check_insert_invalid_arg("x:i32='f'");
        check_insert_invalid_arg("x:opt_i32=''");
    }

    #[test]
    fn should_insert_valid_i64_arg() {
        check_insert_valid_arg("x:i64='9223372036854775807'", i64::MAX);
        check_insert_valid_arg("x:I64='0'", 0_i64);
        check_insert_valid_arg("x:i64='-9223372036854775808'", i64::MIN);
        check_insert_valid_arg("x:opt_i64='-1'", Some(-1_i64));
        check_insert_valid_arg::<Option<i64>>("x:OPT_I64=NULL", None);
    }

    #[test]
    fn should_fail_to_insert_invalid_i64_arg() {
        check_insert_invalid_arg("x:i64='f'");
        check_insert_invalid_arg("x:opt_i64=''");
    }

    #[test]
    fn should_insert_valid_u8_arg() {
        check_insert_valid_arg("x:u8='0'", 0_u8);
        check_insert_valid_arg("x:U8='255'", u8::MAX);
        check_insert_valid_arg("x:opt_u8='1'", Some(1_u8));
        check_insert_valid_arg::<Option<u8>>("x:OPT_U8=null", None);
    }

    #[test]
    fn should_fail_to_insert_invalid_u8_arg() {
        check_insert_invalid_arg("x:u8='f'");
        check_insert_invalid_arg("x:opt_u8=''");
    }

    #[test]
    fn should_insert_valid_u32_arg() {
        check_insert_valid_arg("x:u32='0'", 0_u32);
        check_insert_valid_arg("x:U32='4294967295'", u32::MAX);
        check_insert_valid_arg("x:opt_u32='1'", Some(1_u32));
        check_insert_valid_arg::<Option<u32>>("x:OPT_U32=null", None);
    }

    #[test]
    fn should_fail_to_insert_invalid_u32_arg() {
        check_insert_invalid_arg("x:u32='f'");
        check_insert_invalid_arg("x:opt_u32=''");
    }

    #[test]
    fn should_insert_valid_u64_arg() {
        check_insert_valid_arg("x:u64='0'", 0_u64);
        check_insert_valid_arg("x:U64='18446744073709551615'", u64::MAX);
        check_insert_valid_arg("x:opt_u64='1'", Some(1_u64));
        check_insert_valid_arg::<Option<u64>>("x:OPT_U64=null", None);
    }

    #[test]
    fn should_fail_to_insert_invalid_u64_arg() {
        check_insert_invalid_arg("x:u64='f'");
        check_insert_invalid_arg("x:opt_u64=''");
    }

    #[test]
    fn should_insert_valid_u128_arg() {
        check_insert_valid_arg("x:u128='0'", U128::zero());
        check_insert_valid_arg(
            "x:U128='340282366920938463463374607431768211455'",
            U128::max_value(),
        );
        check_insert_valid_arg("x:opt_u128='1'", Some(U128::from(1)));
        check_insert_valid_arg::<Option<U128>>("x:OPT_U128=null", None);
    }

    #[test]
    fn should_fail_to_insert_invalid_u128_arg() {
        check_insert_invalid_arg("x:u128='f'");
        check_insert_invalid_arg("x:opt_u128=''");
    }

    #[test]
    fn should_insert_valid_u256_arg() {
        check_insert_valid_arg("x:u256='0'", U256::zero());
        check_insert_valid_arg(
            "x:U256='115792089237316195423570985008687907853269984665640564039457584007913129639935'",
            U256::max_value(),
        );
        check_insert_valid_arg("x:opt_u256='1'", Some(U256::from(1)));
        check_insert_valid_arg::<Option<U256>>("x:OPT_U256=null", None);
    }

    #[test]
    fn should_fail_to_insert_invalid_u256_arg() {
        check_insert_invalid_arg("x:u256='f'");
        check_insert_invalid_arg("x:opt_u256=''");
    }

    #[test]
    fn should_insert_valid_u512_arg() {
        check_insert_valid_arg("x:u512='0'", U512::zero());
        check_insert_valid_arg(
            "x:U512='134078079299425970995740249982058461274793658205923933777235614437217640300735\
            46976801874298166903427690031858186486050853753882811946569946433649006084095'",
            U512::max_value(),
        );
        check_insert_valid_arg("x:opt_u512='1'", Some(U512::from(1)));
        check_insert_valid_arg::<Option<U512>>("x:OPT_U512=null", None);
    }

    #[test]
    fn should_fail_to_insert_invalid_u512_arg() {
        check_insert_invalid_arg("x:u512='f'");
        check_insert_invalid_arg("x:opt_u512=''");
    }

    #[test]
    fn should_insert_valid_unit_arg() {
        check_insert_valid_arg("x:unit=''", ());
        check_insert_valid_arg("x:opt_unit=''", Some(()));
        check_insert_valid_arg::<Option<()>>("x:OPT_UNIT=null", None);
    }

    #[test]
    fn should_fail_to_insert_invalid_unit_arg() {
        check_insert_invalid_arg("x:unit='f'");
        check_insert_invalid_arg("x:opt_unit='1'");
    }

    #[test]
    fn should_insert_valid_string_arg() {
        let value = String::from("test \"string");
        check_insert_valid_arg(&format!("x:string='{}'", value), value.clone());
        check_insert_valid_arg(&format!("x:opt_string='{}'", value), Some(value));
        check_insert_valid_arg::<Option<String>>("x:OPT_STRING=null", None);
    }

    #[test]
    fn should_insert_valid_key_arg() {
        let bytes = (1..33).collect::<Vec<_>>();
        let array = <[u8; 32]>::try_from(bytes.as_ref()).unwrap();

        let key_account = Key::Account(AccountHash::new(array));
        let key_hash = Key::Hash(array);
        let key_uref = Key::URef(URef::new(array, AccessRights::NONE));

        for key in &[key_account, key_hash, key_uref] {
            check_insert_valid_arg(&format!("x:key='{}'", key.to_formatted_string()), *key);
            check_insert_valid_arg(
                &format!("x:opt_key='{}'", key.to_formatted_string()),
                Some(*key),
            );
            check_insert_valid_arg::<Option<Key>>("x:OPT_KEY=null", None);
        }
    }

    #[test]
    fn should_fail_to_insert_invalid_key_arg() {
        check_insert_invalid_arg("x:key='f'");
        check_insert_invalid_arg("x:opt_key=''");
    }

    #[test]
    fn should_insert_valid_account_hash_arg() {
        let bytes = (1..33).collect::<Vec<_>>();
        let array = <[u8; 32]>::try_from(bytes.as_ref()).unwrap();
        let value = AccountHash::new(array);
        check_insert_valid_arg(
            &format!("x:account_hash='{}'", value.to_formatted_string()),
            value,
        );
        check_insert_valid_arg(
            &format!("x:opt_account_hash='{}'", value.to_formatted_string()),
            Some(value),
        );
        check_insert_valid_arg::<Option<AccountHash>>("x:OPT_ACCOUNT_HASH=null", None);
    }

    #[test]
    fn should_fail_to_insert_invalid_account_hash_arg() {
        check_insert_invalid_arg("x:account_hash='f'");
        check_insert_invalid_arg("x:account_hash='account-hash-f'");
        check_insert_invalid_arg("x:opt_account_hash=''");
    }

    #[test]
    fn should_insert_valid_uref_arg() {
        let bytes = (1..33).collect::<Vec<_>>();
        let array = <[u8; 32]>::try_from(bytes.as_ref()).unwrap();
        let value = URef::new(array, AccessRights::READ_ADD_WRITE);
        check_insert_valid_arg(&format!("x:uref='{}'", value.to_formatted_string()), value);
        check_insert_valid_arg(
            &format!("x:opt_uref='{}'", value.to_formatted_string()),
            Some(value),
        );
        check_insert_valid_arg::<Option<URef>>("x:OPT_UREF=null", None);
    }

    #[test]
    fn should_fail_to_insert_invalid_uref_arg() {
        check_insert_invalid_arg("x:uref='f'");
        check_insert_invalid_arg("x:uref='uref-f'");
        check_insert_invalid_arg("x:opt_uref=''");
    }

    #[test]
    fn should_insert_valid_public_key_arg() {
        let hex_value = "0119bf44096984cdfe8541bac167dc3b96c85086aa30b6b6cb0c5c38ad703166e1";
        let value = PublicKey::from_hex(hex_value).unwrap();
        check_insert_valid_arg(&format!("x:public_key='{}'", hex_value), value.clone());
        check_insert_valid_arg(&format!("x:opt_public_key='{}'", hex_value), Some(value));
        check_insert_valid_arg::<Option<PublicKey>>("x:OPT_PUBLIC_KEY=null", None);
    }

    #[test]
    fn should_fail_to_insert_invalid_public_key_arg() {
        check_insert_invalid_arg("x:public_key='f'");
        check_insert_invalid_arg("x:opt_public_key=''");
    }

    #[test]
    fn should_insert_valid_byte_list_arg() {
        check_insert_valid_arg("x:byte_list=''", Bytes::new());
        check_insert_valid_arg("x:opt_byte_list=''", Some(Bytes::new()));
        check_insert_valid_arg::<Option<Bytes>>("x:opt_byte_list=null", None);

        let value = Bytes::from(vec![0_u8, 1, 2, 3, 4, 5]);
        let hex_value = base16::encode_upper(&value);

        check_insert_valid_arg(&format!("x:Byte_List='{}'", hex_value), value.clone());
        check_insert_valid_arg(&format!("x:opt_byte_list='{}'", hex_value), Some(value));
        check_insert_valid_arg::<Option<Bytes>>("x:OPT_BYTE_LIST=null", None);
    }

    #[test]
    fn should_fail_to_insert_invalid_byte_list_arg() {
        check_insert_invalid_arg("x:byte_list='fz'");
    }

    #[test]
    fn should_insert_valid_byte_array_arg() {
        check_insert_valid_arg("x:byte_array_0=''", []);
        check_insert_valid_arg("x:opt_byte_array_0=''", Some([]));
        check_insert_valid_arg::<Option<[u8; 0]>>("x:opt_byte_array_0=null", None);

        let value = [0_u8, 1, 2, 3, 4, 5];
        let hex_value = base16::encode_upper(&value);

        check_insert_valid_arg(&format!("x:Byte_Array_6='{}'", hex_value), value);
        check_insert_valid_arg(&format!("x:opt_byte_array_6='{}'", hex_value), Some(value));
        check_insert_valid_arg::<Option<[u8; 6]>>("x:OPT_BYTE_ARRAY_6=null", None);
    }

    #[test]
    fn should_fail_to_insert_invalid_byte_array_arg() {
        check_insert_invalid_arg("x:byte_array_1='fz'");
        check_insert_invalid_arg("x:byte_array_1=''");
        check_insert_invalid_arg("x:byte_array_1='0102'");
    }

    #[test]
    fn should_fail_to_insert_malformed_args() {
        const ARG_BAD_TYPE: &str = "name:wat='false'";
        const ARG_GIBBERISH: &str = "asdf|1234(..)";
        const ARG_UNQUOTED: &str = "name:u32=0"; // value needs single quotes to be valid
        const EMPTY: &str = "";
        const LARGE_2K_INPUT: &str = r#"
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
"#;

        check_insert_invalid_arg(ARG_BAD_TYPE);
        check_insert_invalid_arg(ARG_GIBBERISH);
        check_insert_invalid_arg(ARG_UNQUOTED);
        check_insert_invalid_arg(EMPTY);
        check_insert_invalid_arg(LARGE_2K_INPUT);
    }
}
