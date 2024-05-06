use casper_types::bytesrepr::Bytes;

#[cfg(doc)]
use crate::cli;

/// Container for payment-related arguments used while constructing a `Deploy`.
///
/// ## `payment_args_simple`
///
/// For methods taking `payment_args_simple`, this parameter is the payment contract arguments, in
/// the form `<NAME:TYPE='VALUE'>` or `<NAME:TYPE=null>`.
///
/// It can only be used with the following simple `CLType`s: bool, i32, i64, u8, u32, u64, u128,
/// u256, u512, unit, string, key, account_hash, uref, public_key and `Option` of each of these.
///
/// Example inputs are:
///
/// ```text
/// name_01:bool='false'
/// name_02:i32='-1'
/// name_03:i64='-2'
/// name_04:u8='3'
/// name_05:u32='4'
/// name_06:u64='5'
/// name_07:u128='6'
/// name_08:u256='7'
/// name_09:u512='8'
/// name_10:unit=''
/// name_11:string='a value'
/// key_account_name:key='account-hash-0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20'
/// key_hash_name:key='hash-0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20'
/// key_uref_name:key='uref-0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20-000'
/// account_hash_name:account_hash='account-hash-0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20'
/// uref_name:uref='uref-0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20-007'
/// public_key_name:public_key='0119bf44096984cdfe8541bac167dc3b96c85086aa30b6b6cb0c5c38ad703166e1'
/// ```
///
/// For optional values of any these types, prefix the type with "opt_" and use the term "null"
/// without quotes to specify a None value:
///
/// ```text
/// name_01:opt_bool='true'       # Some(true)
/// name_02:opt_bool='false'      # Some(false)
/// name_03:opt_bool=null         # None
/// name_04:opt_i32='-1'          # Some(-1)
/// name_05:opt_i32=null          # None
/// name_06:opt_unit=''           # Some(())
/// name_07:opt_unit=null         # None
/// name_08:opt_string='a value'  # Some("a value".to_string())
/// name_09:opt_string='null'     # Some("null".to_string())
/// name_10:opt_string=null       # None
/// ```
///
/// To get a list of supported types, call [`cli::simple_args_help::supported_cl_type_list`]. To
/// get this list of examples for supported types, call
/// [`cli::simple_args_help::supported_cl_type_examples`].
///
/// ## `payment_args_json`
///
/// For methods taking `payment_args_json`, this parameter is the payment contract arguments, as a
/// JSON-encoded Array of JSON Objects of the form:
/// ```json
/// [{"name":<String>,"type":<VALUE>,"value":<VALUE>}]
/// ```
///
/// To get comprehensive information and a set of examples, call
/// [`cli::json_args_help::info_and_examples`].
///
/// Example inputs are:
///
/// ```json
/// [{"name":"a","type":"Bool","value":false}]
/// [{"name":"b","type":"I32","value":-1}]
/// [{"name":"c","type":"U64","value":1}]
/// [{"name":"d","type":"U512","value":"20000000000000000000"}]
/// [{"name":"e","type":"Unit","value":null}]
/// [{"name":"f","type":"String","value":"a"}]
/// [{"name":"g","type":"Key","value":"account-hash-201f1e1d1c1b1a191817161514131211100f0e0d0c0b0a090807060504030201"}]
/// [{"name":"h","type":"URef","value":"uref-201f1e1d1c1b1a191817161514131211100f0e0d0c0b0a090807060504030201-007"}]
/// [{"name":"i","type":"PublicKey","value":"017279ea868d185a40ed32ec076807c070de9c0fe986f5418c2aa71478f1e8ddf8"}]
/// [{"name":"j","type":{"Option":"U64"},"value":999}]        # Some(999_u64)
/// [{"name":"k","type":{"Option":"String"},"value":null}]    # None
/// [{"name":"l","type":{"List":{"Option":"U256"}},"value":[1,null,"3"]}]
/// [{"name":"m","type":{"List":"U8"},"value":"0102ff"}]      # for Vec<u8> only, can parse from hex string
/// [{"name":"n","type":{"ByteArray":3},"value":"0114ff"}]
/// [{"name":"o","type":{"ByteArray":3},"value":[1,20,255]}]
/// [{"name":"p","type":{"Result":{"ok":"Bool","err":"U8"}},"value":{"Ok":true}}]
/// [{"name":"q","type":{"Result":{"ok":"Bool","err":"U8"}},"value":{"Err":1}}]
/// [{"name":"r","type":{"Map":{"key":"U8","value":"Bool"}},"value":[{"key":1,"value":true},{"key":2,"value":false}]}]
/// ## for Map with key as String or Number can use an Object rather than an Array:
/// [{"name":"s","type":{"Map":{"key":"U8","value":"Bool"}},"value":{"1":true,"2":false}}]
/// [{"name":"t","type":{"Tuple3":["Bool","U8","String"]},"value":[true,128,"a"]}]
/// ```
///
/// ---
///
/// **Note** while multiple payment args can be specified for a single payment code instance, only
/// one of `payment_args_simple`, or `payment_args_json` may be used.
#[derive(Default, Debug)]
pub struct PaymentStrParams<'a> {
    pub(super) payment_amount: &'a str,
    pub(super) payment_hash: &'a str,
    pub(super) payment_name: &'a str,
    pub(super) payment_package_hash: &'a str,
    pub(super) payment_package_name: &'a str,
    pub(super) payment_path: &'a str,
    pub(super) payment_bytes: Bytes,
    pub(super) payment_args_simple: Vec<&'a str>,
    pub(super) payment_args_json: &'a str,
    pub(super) payment_version: &'a str,
    pub(super) payment_entry_point: &'a str,
}

impl<'a> PaymentStrParams<'a> {
    /// Constructs a `PaymentStrParams` using a payment smart contract file.
    ///
    /// * `payment_path` is the path to the compiled Wasm payment code.
    /// * See the struct docs for a description of [`payment_args_simple`](#payment_args_simple),
    ///   [`payment_args_json`](#payment_args_json)
    pub fn with_path(
        payment_path: &'a str,
        payment_args_simple: Vec<&'a str>,
        payment_args_json: &'a str,
    ) -> Self {
        Self {
            payment_path,
            payment_args_simple,
            payment_args_json,
            ..Default::default()
        }
    }

    /// Constructs a `PaymentStrParams` using a payment amount.
    ///
    /// `payment_amount` uses the standard-payment system contract rather than custom payment Wasm.
    /// The value is the 'amount' arg of the standard-payment contract.
    pub fn with_amount(payment_amount: &'a str) -> Self {
        Self {
            payment_amount,
            ..Default::default()
        }
    }

    /// Constructs a `PaymentStrParams` using a stored contract's name.
    ///
    /// * `payment_name` is the name of the stored contract (associated with the executing account)
    ///   to be called as the payment.
    /// * `payment_entry_point` is the name of the method that will be used when calling the payment
    ///   contract.
    /// * See the struct docs for a description of [`payment_args_simple`](#payment_args_simple),
    ///   [`payment_args_json`](#payment_args_json) and
    ///   [`payment_args_complex`](#payment_args_complex).
    pub fn with_name(
        payment_name: &'a str,
        payment_entry_point: &'a str,
        payment_args_simple: Vec<&'a str>,
        payment_args_json: &'a str,
    ) -> Self {
        Self {
            payment_name,
            payment_args_simple,
            payment_args_json,
            payment_entry_point,
            ..Default::default()
        }
    }

    /// Constructs a `PaymentStrParams` using a stored contract's hex-encoded hash.
    ///
    /// * `payment_hash` is the hex-encoded hash of the stored contract to be called as the payment.
    /// * `payment_entry_point` is the name of the method that will be used when calling the payment
    ///   contract.
    /// * See the struct docs for a description of [`payment_args_simple`](#payment_args_simple),
    ///   [`payment_args_json`](#payment_args_json) and
    ///   [`payment_args_complex`](#payment_args_complex).
    pub fn with_hash(
        payment_hash: &'a str,
        payment_entry_point: &'a str,
        payment_args_simple: Vec<&'a str>,
        payment_args_json: &'a str,
    ) -> Self {
        Self {
            payment_hash,
            payment_args_simple,
            payment_args_json,
            payment_entry_point,
            ..Default::default()
        }
    }

    /// Constructs a `PaymentStrParams` using a stored contract's package name.
    ///
    /// * `payment_package_name` is the name of the stored package to be called as the payment.
    /// * `payment_version` is the version of the called payment contract. The latest will be used
    ///   if `payment_version` is empty.
    /// * `payment_entry_point` is the name of the method that will be used when calling the payment
    ///   contract.
    /// * See the struct docs for a description of [`payment_args_simple`](#payment_args_simple),
    ///   [`payment_args_json`](#payment_args_json) and
    ///   [`payment_args_complex`](#payment_args_complex).
    pub fn with_package_name(
        payment_package_name: &'a str,
        payment_version: &'a str,
        payment_entry_point: &'a str,
        payment_args_simple: Vec<&'a str>,
        payment_args_json: &'a str,
    ) -> Self {
        Self {
            payment_package_name,
            payment_args_simple,
            payment_args_json,
            payment_version,
            payment_entry_point,
            ..Default::default()
        }
    }

    /// Constructs a `PaymentStrParams` using a stored contract's package hash.
    ///
    /// * `payment_package_hash` is the hex-encoded hash of the stored package to be called as the
    ///   payment.
    /// * `payment_version` is the version of the called payment contract. The latest will be used
    ///   if `payment_version` is empty.
    /// * `payment_entry_point` is the name of the method that will be used when calling the payment
    ///   contract.
    /// * See the struct docs for a description of [`payment_args_simple`](#payment_args_simple),
    ///   [`payment_args_json`](#payment_args_json) and
    ///   [`payment_args_complex`](#payment_args_complex).
    pub fn with_package_hash(
        payment_package_hash: &'a str,
        payment_version: &'a str,
        payment_entry_point: &'a str,
        payment_args_simple: Vec<&'a str>,
        payment_args_json: &'a str,
    ) -> Self {
        Self {
            payment_package_hash,
            payment_args_simple,
            payment_args_json,
            payment_version,
            payment_entry_point,
            ..Default::default()
        }
    }
}
