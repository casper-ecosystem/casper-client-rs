/// Container for `Transaction` construction options.
///
///
/// ## `session_args_simple`
///
/// For methods taking `session_args_simple`, this parameter is the session contract arguments, in
/// the form `<NAME:TYPE='VALUE'>` or `<NAME:TYPE=null>`.
///
/// There are further details in
/// [the docs for the equivalent
/// `payment_args_simple`](struct.PaymentStrParams.html#payment_args_simple).
///
/// ## `session_args_json`
///
/// For methods taking `session_args_json`, this parameter is the session contract arguments, as a
/// JSON-encoded Array of JSON Objects of the form:
/// ```json
/// [{"name":<String>,"type":<VALUE>,"value":<VALUE>}]
/// ```
///
/// There are further details in
/// [the docs for the equivalent `payment_args_json`](struct.PaymentStrParams.html#payment_args_json).
#[derive(Default, Debug, Clone)]
pub struct TransactionStrParams<'a> {
    /// Path to secret key file.
    ///
    /// If `secret_key` is empty, the new transaction will not be signed and will need to be signed (e.g.
    /// via [`sign_transaction_file`](super::sign_transaction_file)) at least once in order to be made valid.
    pub secret_key: &'a str,
    /// RFC3339-like formatted timestamp. e.g. `2018-02-16T00:31:37Z`.
    ///
    /// If `timestamp` is empty, the current time will be used. Note that timestamp is UTC, not
    /// local.
    ///
    /// See [`humantime::parse_rfc3339_weak`] for more information.
    pub timestamp: &'a str,
    /// Time that the `transaction` will remain valid for.
    ///
    /// A `transaction` can only be included in a `Block` between `timestamp` and `timestamp + ttl`.
    /// Input examples: '1hr 12min', '30min 50sec', '1day'.
    ///
    /// See [`humantime::parse_duration`] for more information.
    pub ttl: &'a str,
    /// Name of the chain, to avoid the `transaction` from being accidentally or maliciously included in
    /// a different chain.
    pub chain_name: &'a str,
    /// The hex-encoded public key, account hash, or entity address of the account context under which
    /// the session code will be executed.
    ///
    /// If `initiator_addr` is empty, the initiator address will be derived from the provided
    /// `secret_key`.  It is an error for both fields to be empty.
    pub initiator_addr: String,
    /// Simple session args for use in the transaction
    pub session_args_simple: Vec<&'a str>,
    /// Session args in json for use with the transaction
    pub session_args_json: &'a str,
    /// The pricing mode to use with the transaction
    pub pricing_mode: &'a str,
    /// User-specified additional computation factor for "fixed" pricing_mode (minimum 0)
    /// if "0" is provided, no additional logic is applied to the computation limit.
    pub additional_computation_factor: &'a str,
    /// The optional output path for the transaction, if writing it to a file.
    pub output_path: &'a str,
    /// The payment amount for executing the transaction
    pub payment_amount: &'a str,
    /// User-specified gas_price tolerance.
    pub gas_price_tolerance: &'a str,
    /// The digest of a previous transaction that represents the receipt for the current transaction.
    pub receipt: &'a str,
    /// Standard payment.
    pub standard_payment: &'a str,
}
