//! Functions for use in help commands.

use std::convert::TryFrom;

use itertools::Itertools;

use casper_types::{account::AccountHash, AccessRights, AsymmetricType, Key, PublicKey, URef};

use super::{PREFIX_FOR_OPTION, SUPPORTED_TYPES};

/// Returns a list of `CLType`s able to be passed as a string for use as payment code or session
/// code args.
pub fn supported_cl_type_list() -> String {
    format!(
        "{}, {}",
        SUPPORTED_TYPES
            .iter()
            .map(|(simple_type, _parser)| simple_type)
            .join(", "),
        SUPPORTED_TYPES
            .iter()
            .map(|(simple_type, _parser)| PREFIX_FOR_OPTION.to_string() + simple_type)
            .join(", ")
    )
}

/// Returns a string listing examples of the format required when passing in payment code or
/// session code args.
pub fn supported_cl_type_examples() -> String {
    let bytes = (1..33).collect::<Vec<_>>();
    let array = <[u8; 32]>::try_from(bytes.as_ref()).unwrap();

    format!(
        r#""name_01:bool='false'"
"name_02:i32='-1'"
"name_03:i64='-2'"
"name_04:u8='3'"
"name_05:u32='4'"
"name_06:u64='5'"
"name_07:u128='6'"
"name_08:u256='7'"
"name_09:u512='8'"
"name_10:unit=''"
"name_11:string='a value'"
"key_account_name:key='{}'"
"key_hash_name:key='{}'"
"key_uref_name:key='{}'"
"account_hash_name:account_hash='{}'"
"uref_name:uref='{}'"
"public_key_name:public_key='{}'"
"byte_list_name:byte_list='010203'"            # variable-length list of bytes, i.e. CLType::List(CLType::U8)
"byte_array_5_name:byte_array_5='0102030405'"  # fixed-length array of bytes, in this example CLType::ByteArray(5)
"byte_array_32_name:byte_array_32='{}'"

Optional values of all of these types can also be specified.
Prefix the type with "opt_" and use the term "null" without quotes to specify a None value:
"name_01:opt_bool='true'"       # Some(true)
"name_02:opt_bool='false'"      # Some(false)
"name_03:opt_bool=null"         # None
"name_04:opt_i32='-1'"          # Some(-1)
"name_05:opt_i32=null"          # None
"name_06:opt_unit=''"           # Some(())
"name_07:opt_unit=null"         # None
"name_08:opt_string='a value'"  # Some("a value".to_string())
"name_09:opt_string='null'"     # Some("null".to_string())
"name_10:opt_string=null"       # None
"#,
        Key::Account(AccountHash::new(array)).to_formatted_string(),
        Key::Hash(array).to_formatted_string(),
        Key::URef(URef::new(array, AccessRights::NONE)).to_formatted_string(),
        AccountHash::new(array).to_formatted_string(),
        URef::new(array, AccessRights::READ_ADD_WRITE).to_formatted_string(),
        PublicKey::from_hex("0119bf44096984cdfe8541bac167dc3b96c85086aa30b6b6cb0c5c38ad703166e1")
            .unwrap()
            .to_hex(),
        base16::encode_lower(&bytes),
    )
}
