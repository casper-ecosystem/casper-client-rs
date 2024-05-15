//! This module contains structs and helpers which are used by multiple subcommands related to
//! creating deploys.

use std::process;

use clap::{Arg, ArgAction, ArgGroup, ArgMatches, Command};

use casper_client::cli::{json_args_help, simple_args_help, PaymentStrParams, SessionStrParams};

use crate::common;

const SESSION_ARG_GROUP: &str = "session-args";

/// This struct defines the order in which the args are shown for this subcommand's help message.
pub(super) enum DisplayOrder {
    ShowSimpleArgExamples,
    ShowJsonArgExamples,
    Verbose,
    NodeAddress,
    RpcId,
    SpeculativeExec,
    SecretKey,
    Input,
    Output,
    Force,
    GasPrice,
    TransferAmount,
    TransferTargetAccount,
    TransferId,
    Timestamp,
    Ttl,
    ChainName,
    SessionCode,
    SessionArgSimple,
    SessionArgsJson,
    SessionHash,
    SessionName,
    SessionPackageHash,
    SessionPackageName,
    SessionEntryPoint,
    SessionVersion,
    SessionTransfer,
    SessionAccount,
    StandardPayment,
    PaymentCode,
    PaymentArgSimple,
    PaymentArgsJson,
    PaymentHash,
    PaymentName,
    PaymentPackageHash,
    PaymentPackageName,
    PaymentEntryPoint,
    PaymentVersion,
}

/// Handles providing the arg for and executing the show-simple-arg-examples option.
pub(super) mod show_simple_arg_examples {
    use super::*;

    pub(in crate::deploy) const ARG_NAME: &str = "show-simple-arg-examples";
    const ARG_ALIAS: &str = "show-arg-examples";
    const ARG_SHORT: char = 'e';
    const ARG_HELP: &str =
        "If passed, all other options are ignored and a set of examples of session-/payment-args \
        is printed";

    pub(in crate::deploy) fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .alias(ARG_ALIAS)
            .long(ARG_NAME)
            .short(ARG_SHORT)
            .required(false)
            .action(ArgAction::SetTrue)
            .help(ARG_HELP)
            .display_order(DisplayOrder::ShowSimpleArgExamples as usize)
    }

    pub(in crate::deploy) fn get(matches: &ArgMatches) -> bool {
        if let Some(true) = matches.get_one::<bool>(ARG_NAME) {
            println!("Examples for passing values via --session-arg or --payment-arg:");
            println!("{}", simple_args_help::supported_cl_type_examples());
            return true;
        }

        false
    }
}

/// Handles providing the arg for and executing the show-json-arg-examples option.
pub(super) mod show_json_args_examples {
    use super::*;

    pub(in crate::deploy) const ARG_NAME: &str = "show-json-args-examples";
    const ARG_SHORT: char = 'j';
    const ARG_HELP: &str = "If passed, all other options are ignored and a set of examples of \
        session-/payment-args-json is printed";

    pub(in crate::deploy) fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .short(ARG_SHORT)
            .required(false)
            .action(ArgAction::SetTrue)
            .help(ARG_HELP)
            .display_order(DisplayOrder::ShowJsonArgExamples as usize)
    }

    pub(in crate::deploy) fn get(matches: &ArgMatches) -> bool {
        if let Some(true) = matches.get_one::<bool>(ARG_NAME) {
            println!("Examples for passing values via --session-args-json or --payment-args-json:");
            println!();
            println!("{}", json_args_help::info_and_examples());
            return true;
        }

        false
    }
}

pub(super) fn session_str_params(matches: &ArgMatches) -> SessionStrParams<'_> {
    let session_args_simple = arg_simple::session::get(matches);
    let session_args_json = args_json::session::get(matches);
    if is_session_transfer::get(matches) {
        return SessionStrParams::with_transfer(session_args_simple, session_args_json);
    }
    if let Some(session_path) = session_path::get(matches) {
        return SessionStrParams::with_path(session_path, session_args_simple, session_args_json);
    }
    let session_entry_point = session_entry_point::get(matches);
    if let Some(session_hash) = session_hash::get(matches) {
        return SessionStrParams::with_hash(
            session_hash,
            session_entry_point,
            session_args_simple,
            session_args_json,
        );
    }
    if let Some(session_name) = session_name::get(matches) {
        return SessionStrParams::with_name(
            session_name,
            session_entry_point,
            session_args_simple,
            session_args_json,
        );
    }
    let session_version = session_version::get(matches);
    if let Some(session_package_hash) = session_package_hash::get(matches) {
        return SessionStrParams::with_package_hash(
            session_package_hash,
            session_version,
            session_entry_point,
            session_args_simple,
            session_args_json,
        );
    }
    if let Some(session_package_name) = session_package_name::get(matches) {
        return SessionStrParams::with_package_name(
            session_package_name,
            session_version,
            session_entry_point,
            session_args_simple,
            session_args_json,
        );
    }
    unreachable!("clap arg groups and parsing should prevent this")
}

pub(super) fn payment_str_params(matches: &ArgMatches) -> PaymentStrParams<'_> {
    if let Some(payment_amount) = standard_payment_amount::get(matches) {
        return PaymentStrParams::with_amount(payment_amount);
    }
    let payment_args_simple = arg_simple::payment::get(matches);
    let payment_args_json = args_json::payment::get(matches);
    if let Some(payment_path) = payment_path::get(matches) {
        return PaymentStrParams::with_path(payment_path, payment_args_simple, payment_args_json);
    }
    let payment_entry_point = payment_entry_point::get(matches);
    if let Some(payment_hash) = payment_hash::get(matches) {
        return PaymentStrParams::with_hash(
            payment_hash,
            payment_entry_point,
            payment_args_simple,
            payment_args_json,
        );
    }
    if let Some(payment_name) = payment_name::get(matches) {
        return PaymentStrParams::with_name(
            payment_name,
            payment_entry_point,
            payment_args_simple,
            payment_args_json,
        );
    }
    let payment_version = payment_version::get(matches);
    if let Some(payment_package_hash) = payment_package_hash::get(matches) {
        return PaymentStrParams::with_package_hash(
            payment_package_hash,
            payment_version,
            payment_entry_point,
            payment_args_simple,
            payment_args_json,
        );
    }
    if let Some(payment_package_name) = payment_package_name::get(matches) {
        return PaymentStrParams::with_package_name(
            payment_package_name,
            payment_version,
            payment_entry_point,
            payment_args_simple,
            payment_args_json,
        );
    }
    unreachable!("clap arg groups and parsing should prevent this")
}

/// Handles providing the arg for speculative execution.
pub(super) mod speculative_exec {
    use super::*;

    const ARG_NAME: &str = "speculative-exec";
    const ARG_VALUE_NAME: &str = "HEX STRING OR INTEGER";
    const ARG_HELP: &str =
        "If the receiving node supports this, execution of the deploy will only be attempted on \
        that single node. Full validation of the deploy is not performed, and successful execution \
        at the given global state is no guarantee that the deploy will be able to be successfully \
        executed if put to the network, nor should execution costs be expected to be identical. \
        Optionally provide the hex-encoded block hash or height of the block to specify the global \
        state on which to execute";
    const DEFAULT_MISSING_VALUE: &str = "";

    pub(in crate::deploy) fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .num_args(0..=1)
            .default_missing_value(DEFAULT_MISSING_VALUE)
            .help(ARG_HELP)
            .display_order(DisplayOrder::SpeculativeExec as usize)
    }

    // get: The command line posibilities are encoded by a combination of option and &str
    // None represents no --speculative-exec argument at all
    // Some("") represents a --speculative-exec with no/empty argument
    // Some(block_identifier) represents "--speculative-exec block_identifier"
    pub(in crate::deploy) fn get(matches: &ArgMatches) -> Option<&str> {
        matches.get_one::<String>(ARG_NAME).map(String::as_str)
    }
}

/// Handles providing the arg for and retrieval of the timestamp.
pub(super) mod timestamp {
    use super::*;

    const ARG_NAME: &str = "timestamp";
    const ARG_VALUE_NAME: &str = "TIMESTAMP";
    const ARG_HELP: &str =
        "RFC3339-like formatted timestamp, e.g. '2018-02-16 00:31:37'. If not provided, the \
        current time will be used. Note that timestamp is UTC, not local. See \
        https://docs.rs/humantime/latest/humantime/fn.parse_rfc3339_weak.html for more \
        information.";

    pub(in crate::deploy) fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::Timestamp as usize)
    }

    pub(in crate::deploy) fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default()
    }
}

/// Handles providing the arg for and retrieval of the time to live.
pub(super) mod ttl {
    use super::*;

    const ARG_NAME: &str = "ttl";
    const ARG_VALUE_NAME: &str = "DURATION";
    const ARG_DEFAULT: &str = "30min";
    const ARG_HELP: &str =
        "Time that the deploy will remain valid for. A deploy can only be included in a block \
        between `timestamp` and `timestamp + ttl`. Input examples: '1hr 12min', '30min 50sec', \
        '1day'. For all options, see \
        https://docs.rs/humantime/latest/humantime/fn.parse_duration.html";

    pub(in crate::deploy) fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .default_value(ARG_DEFAULT)
            .help(ARG_HELP)
            .display_order(DisplayOrder::Ttl as usize)
    }

    pub(in crate::deploy) fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default()
    }
}

/// Handles providing the arg for and retrieval of the chain name.
pub(super) mod chain_name {
    use super::*;

    const ARG_NAME: &str = "chain-name";
    const ARG_VALUE_NAME: &str = "NAME";
    const ARG_HELP: &str =
        "Name of the chain, to avoid the deploy from being accidentally or maliciously included in \
        a different chain";

    pub(in crate::deploy) fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .required_unless_present(show_simple_arg_examples::ARG_NAME)
            .required_unless_present(show_json_args_examples::ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::ChainName as usize)
    }

    pub(in crate::deploy) fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_else(|| panic!("should have {} arg", ARG_NAME))
    }
}

/// Handles providing the arg for and retrieval of the session code bytes.
pub(super) mod session_path {
    use super::*;

    pub(super) const ARG_NAME: &str = "session-path";
    const ARG_SHORT: char = 's';
    const ARG_VALUE_NAME: &str = common::ARG_PATH;
    const ARG_HELP: &str = "Path to the compiled Wasm session code";

    pub(in crate::deploy) fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .short(ARG_SHORT)
            .long(ARG_NAME)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::SessionCode as usize)
    }

    pub(in crate::deploy) fn get(matches: &ArgMatches) -> Option<&str> {
        matches.get_one::<String>(ARG_NAME).map(String::as_str)
    }
}

/// Handles providing the arg for and retrieval of simple session and payment args.
pub(super) mod arg_simple {
    use super::*;
    use once_cell::sync::Lazy;

    const ARG_VALUE_NAME: &str = r#""NAME:TYPE='VALUE'" OR "NAME:TYPE=null""#;

    static ARG_HELP: Lazy<String> = Lazy::new(|| {
        format!(
            "For simple CLTypes, a named and typed arg which is passed to the Wasm code. To see \
            an example for each type, run '--{}'. This arg can be repeated to pass multiple named, \
            typed args, but can only be used for the following types: {}",
            show_simple_arg_examples::ARG_NAME,
            simple_args_help::supported_cl_type_list()
        )
    });

    pub(in crate::deploy) mod session {
        use super::*;

        pub const ARG_NAME: &str = "session-arg";
        const ARG_SHORT: char = 'a';

        pub fn arg() -> Arg {
            super::arg(ARG_NAME, DisplayOrder::SessionArgSimple as usize).short(ARG_SHORT)
        }

        pub fn get(matches: &ArgMatches) -> Vec<&str> {
            matches
                .get_many::<String>(ARG_NAME)
                .unwrap_or_default()
                .map(|simple_session_arg| simple_session_arg.as_str())
                .collect()
        }
    }

    pub(in crate::deploy) mod payment {
        use super::*;

        pub const ARG_NAME: &str = "payment-arg";

        pub fn arg() -> Arg {
            super::arg(ARG_NAME, DisplayOrder::PaymentArgSimple as usize)
        }

        pub fn get(matches: &ArgMatches) -> Vec<&str> {
            matches
                .get_many::<String>(ARG_NAME)
                .unwrap_or_default()
                .map(|simple_payment_arg| simple_payment_arg.as_str())
                .collect()
        }
    }

    fn arg(name: &'static str, order: usize) -> Arg {
        Arg::new(name)
            .long(name)
            .required(false)
            .action(ArgAction::Append)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP.as_str())
            .display_order(order)
    }
}

pub(super) mod gas_price {
    use super::*;
    pub(crate) const ARG_NAME: &str = "gas-price";

    const ARG_VALUE_NAME: &str = common::ARG_INTEGER;

    const ARG_SHORT: char = 'g';
    const ARG_HELP: &str =
        "The maximum gas price that the user is willing to pay for the transaction";

    pub(crate) fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .short(ARG_SHORT)
            .required(true)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::GasPrice as usize)
    }

    pub fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default()
    }
}


/// Handles providing the arg for and retrieval of JSON session and payment args.
pub(super) mod args_json {
    use super::*;
    use once_cell::sync::Lazy;

    const ARG_VALUE_NAME: &str = "JSON ARRAY";

    static ARG_HELP: Lazy<String> = Lazy::new(|| {
        format!(
            "A JSON Array of named and typed args which is passed to the Wasm code. To see \
            examples, run '--{}'.",
            show_json_args_examples::ARG_NAME,
        )
    });

    pub(in crate::deploy) mod session {
        use super::*;

        pub const ARG_NAME: &str = "session-args-json";

        pub fn arg() -> Arg {
            super::arg(ARG_NAME, DisplayOrder::SessionArgsJson as usize)
        }

        pub fn get(matches: &ArgMatches) -> &str {
            matches
                .get_one::<String>(ARG_NAME)
                .map(String::as_str)
                .unwrap_or_default()
        }
    }

    pub(in crate::deploy) mod payment {
        use super::*;

        pub const ARG_NAME: &str = "payment-args-json";

        pub fn arg() -> Arg {
            super::arg(ARG_NAME, DisplayOrder::PaymentArgsJson as usize)
        }

        pub fn get(matches: &ArgMatches) -> &str {
            matches
                .get_one::<String>(ARG_NAME)
                .map(String::as_str)
                .unwrap_or_default()
        }
    }

    fn arg(name: &'static str, order: usize) -> Arg {
        Arg::new(name)
            .long(name)
            .required(false)
            .action(ArgAction::Append)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP.as_str())
            .display_order(order)
    }
}

/// Handles providing the arg for and retrieval of the payment code bytes.
pub(super) mod payment_path {
    use super::*;

    pub(in crate::deploy) const ARG_NAME: &str = "payment-path";
    const ARG_VALUE_NAME: &str = common::ARG_PATH;
    const ARG_HELP: &str = "Path to the compiled Wasm payment code";

    pub(in crate::deploy) fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::PaymentCode as usize)
    }

    pub fn get(matches: &ArgMatches) -> Option<&str> {
        matches.get_one::<String>(ARG_NAME).map(String::as_str)
    }
}

/// Handles providing the arg for and retrieval of the payment-amount arg.
pub(super) mod standard_payment_amount {
    use super::*;

    pub(in crate::deploy) const ARG_NAME: &str = "payment-amount";
    const ARG_VALUE_NAME: &str = "AMOUNT";
    const ARG_SHORT: char = 'p';
    const ARG_HELP: &str =
        "If provided, uses the standard-payment system contract rather than custom payment Wasm. \
        The value is the 'amount' arg of the standard-payment contract. This arg is incompatible \
        with all other --payment-xxx args";

    pub(in crate::deploy) fn arg(default_value: Option<&'static str>) -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .short(ARG_SHORT)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::StandardPayment as usize)
            .default_value(default_value)
    }

    pub fn get(matches: &ArgMatches) -> Option<&str> {
        matches.get_one::<String>(ARG_NAME).map(String::as_str)
    }
}

pub(super) fn apply_common_creation_options(
    subcommand: Command,
    require_secret_key: bool,
    include_node_address: bool,
) -> Command {
    let mut subcommand = subcommand
        .next_line_help(true)
        .arg(show_simple_arg_examples::arg())
        .arg(show_json_args_examples::arg())
        .group(
            ArgGroup::new("show-examples")
                .arg(show_simple_arg_examples::ARG_NAME)
                .arg(show_json_args_examples::ARG_NAME),
        );

    if include_node_address {
        subcommand = subcommand.arg(
            common::node_address::arg(DisplayOrder::NodeAddress as usize)
                .required_unless_present(show_simple_arg_examples::ARG_NAME)
                .required_unless_present(show_json_args_examples::ARG_NAME),
        );
    }

    let secret_key_arg = if require_secret_key {
        common::secret_key::arg(DisplayOrder::SecretKey as usize, "")
            .required_unless_present(show_simple_arg_examples::ARG_NAME)
            .required_unless_present(show_json_args_examples::ARG_NAME)
    } else {
        common::secret_key::arg(
            DisplayOrder::SecretKey as usize,
            ". If not provided, the deploy will not be signed and will remain invalid until \
            signed, for example by running the `sign-deploy` subcommand.",
        )
    };

    subcommand = subcommand
        .arg(secret_key_arg)
        .arg(timestamp::arg())
        .arg(ttl::arg())
        .arg(chain_name::arg())
        .arg(session_account::arg(DisplayOrder::SessionAccount as usize));
    subcommand
}

pub(super) fn apply_common_session_options(subcommand: Command) -> Command {
    subcommand
        .arg(arg_simple::session::arg())
        .arg(args_json::session::arg())
        // Group the session-arg args so only one style is used to ensure consistent ordering.
        .group(
            ArgGroup::new(SESSION_ARG_GROUP)
                .arg(arg_simple::session::ARG_NAME)
                .arg(args_json::session::ARG_NAME)
                .required(false),
        )
        .arg(is_session_transfer::arg())
        .arg(session_path::arg())
        .arg(session_entry_point::arg())
        .arg(session_hash::arg())
        .arg(session_name::arg())
        .arg(session_version::arg())
        .arg(session_package_hash::arg())
        .arg(session_package_name::arg())
        .group(
            ArgGroup::new("session")
                .arg(session_path::ARG_NAME)
                .arg(session_package_hash::ARG_NAME)
                .arg(session_package_name::ARG_NAME)
                .arg(is_session_transfer::ARG_NAME)
                .arg(session_hash::ARG_NAME)
                .arg(session_name::ARG_NAME)
                .arg(show_simple_arg_examples::ARG_NAME)
                .arg(show_json_args_examples::ARG_NAME)
                .required(false),
        )
        .group(
            // This group duplicates all the args in the "session" and "show-examples" groups, but
            // ensures at least one of them are provided.
            ArgGroup::new("session-and-show-examples")
                .arg(is_session_transfer::ARG_NAME)
                .arg(session_path::ARG_NAME)
                .arg(session_hash::ARG_NAME)
                .arg(session_name::ARG_NAME)
                .arg(session_package_hash::ARG_NAME)
                .arg(session_package_name::ARG_NAME)
                .arg(show_simple_arg_examples::ARG_NAME)
                .arg(show_json_args_examples::ARG_NAME)
                .multiple(true)
                .required(true),
        )
}

pub(crate) fn apply_common_payment_options(
    subcommand: Command,
    default_amount: Option<&'static str>,
) -> Command {
    subcommand
        .arg(arg_simple::payment::arg())
        .arg(args_json::payment::arg())
        .group(
            // Ensure these three forms of inputting payment-args are mutually exclusive.
            ArgGroup::new("payment-args")
                .arg(arg_simple::payment::ARG_NAME)
                .arg(args_json::payment::ARG_NAME)
                .required(false),
        )
        .arg(standard_payment_amount::arg(default_amount))
        .arg(payment_path::arg())
        .arg(payment_entry_point::arg())
        .arg(payment_hash::arg())
        .arg(payment_name::arg())
        .arg(payment_version::arg())
        .arg(payment_package_hash::arg())
        .arg(payment_package_name::arg())
        .group(
            ArgGroup::new("payment")
                .arg(standard_payment_amount::ARG_NAME)
                .arg(payment_path::ARG_NAME)
                .arg(payment_package_hash::ARG_NAME)
                .arg(payment_package_name::ARG_NAME)
                .arg(payment_hash::ARG_NAME)
                .arg(payment_name::ARG_NAME)
                .arg(show_simple_arg_examples::ARG_NAME)
                .arg(show_json_args_examples::ARG_NAME)
                .required(false),
        )
        .group(
            // This group duplicates all the args in the "payment" and "show-examples" groups, but
            // ensures at least one of them are provided.
            ArgGroup::new("payment-and-show-example")
                .arg(standard_payment_amount::ARG_NAME)
                .arg(payment_path::ARG_NAME)
                .arg(payment_hash::ARG_NAME)
                .arg(payment_name::ARG_NAME)
                .arg(payment_package_hash::ARG_NAME)
                .arg(payment_package_name::ARG_NAME)
                .arg(show_simple_arg_examples::ARG_NAME)
                .arg(show_json_args_examples::ARG_NAME)
                .multiple(true)
                .required(true),
        )
}

pub(super) fn show_simple_arg_examples_and_exit_if_required(matches: &ArgMatches) {
    // If we printed the arg examples, exit the process.
    if show_simple_arg_examples::get(matches) {
        process::exit(0);
    }
}

pub(super) fn show_json_args_examples_and_exit_if_required(matches: &ArgMatches) {
    // If we printed the arg examples, exit the process.
    if show_json_args_examples::get(matches) {
        process::exit(0);
    }
}

pub(super) mod output {
    use super::*;

    const ARG_NAME: &str = "output";
    const ARG_SHORT: char = 'o';
    const ARG_VALUE_NAME: &str = common::ARG_PATH;
    const ARG_HELP: &str =
        "Path to output deploy file. If omitted, defaults to stdout. If the file already exists, \
        the command will fail unless '--force' is also specified";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .required(false)
            .long(ARG_NAME)
            .short(ARG_SHORT)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::Output as usize)
    }

    pub fn get(matches: &ArgMatches) -> Option<&str> {
        matches.get_one::<String>(ARG_NAME).map(String::as_str)
    }
}

pub(super) mod input {
    use super::*;

    const ARG_NAME: &str = "input";
    const ARG_SHORT: char = 'i';
    const ARG_VALUE_NAME: &str = common::ARG_PATH;
    const ARG_HELP: &str = "Path to input deploy file";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .required(true)
            .long(ARG_NAME)
            .short(ARG_SHORT)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::Input as usize)
    }

    pub fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_else(|| panic!("should have {} arg", ARG_NAME))
    }
}

pub(super) mod session_account {
    use super::*;
    use casper_client::cli::CliError;

    pub const ARG_NAME: &str = "session-account";
    const ARG_VALUE_NAME: &str = "FORMATTED STRING or PATH";
    const ARG_HELP: &str =
        "The hex-encoded public key of the account context under which the session code will be \
        executed. This must be a properly formatted public key. The public key may instead be read \
        in from a file, in which case enter the path to the file as the --session-account \
        argument. The file should be one of the two public key files generated via the `keygen` \
        subcommand; \"public_key_hex\" or \"public_key.pem\".  If not provided, the public key of \
        the account will be derived from the key passed via --secret-key";

    pub fn arg(order: usize) -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .required_unless_present_any([
                common::secret_key::ARG_NAME,
                show_simple_arg_examples::ARG_NAME,
                show_json_args_examples::ARG_NAME,
            ])
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(order)
    }

    pub fn get(matches: &ArgMatches) -> Result<String, CliError> {
        let value = matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default();
        common::public_key::try_read_from_file(value)
    }
}

pub(super) mod session_name {
    use super::*;

    pub const ARG_NAME: &str = "session-name";
    const ARG_VALUE_NAME: &str = "NAME";
    const ARG_HELP: &str =
        "Name of the stored contract (associated with the executing account) to be called as the \
     session";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(false)
            .requires(session_entry_point::ARG_NAME)
            .display_order(DisplayOrder::SessionName as usize)
    }

    pub fn get(matches: &ArgMatches) -> Option<&str> {
        matches.get_one::<String>(ARG_NAME).map(String::as_str)
    }
}

pub(super) mod is_session_transfer {
    use super::*;

    pub const ARG_NAME: &str = "is-session-transfer";
    const ARG_HELP: &str = "Use this flag if you want to make this a transfer.";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .action(ArgAction::SetTrue)
            .help(ARG_HELP)
            .required(false)
            .requires(SESSION_ARG_GROUP)
            .display_order(DisplayOrder::SessionTransfer as usize)
    }

    pub fn get(matches: &ArgMatches) -> bool {
        matches
            .get_one::<bool>(ARG_NAME)
            .copied()
            .unwrap_or_default()
    }
}

pub(super) mod session_hash {
    use super::*;

    pub const ARG_NAME: &str = "session-hash";
    const ARG_VALUE_NAME: &str = common::ARG_HEX_STRING;
    const ARG_HELP: &str = "Hex-encoded hash of the stored contract to be called as the session";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(false)
            .requires(session_entry_point::ARG_NAME)
            .display_order(DisplayOrder::SessionHash as usize)
    }

    pub fn get(matches: &ArgMatches) -> Option<&str> {
        matches.get_one::<String>(ARG_NAME).map(String::as_str)
    }
}

pub(super) mod session_package_hash {
    use super::*;

    pub const ARG_NAME: &str = "session-package-hash";
    const ARG_VALUE_NAME: &str = common::ARG_HEX_STRING;
    const ARG_HELP: &str = "Hex-encoded hash of the stored package to be called as the session";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(false)
            .requires(session_entry_point::ARG_NAME)
            .display_order(DisplayOrder::SessionPackageHash as usize)
    }

    pub fn get(matches: &ArgMatches) -> Option<&str> {
        matches.get_one::<String>(ARG_NAME).map(String::as_str)
    }
}

pub(super) mod session_package_name {
    use super::*;

    pub const ARG_NAME: &str = "session-package-name";
    const ARG_VALUE_NAME: &str = "NAME";
    const ARG_HELP: &str = "Name of the stored package to be called as the session";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(false)
            .requires(session_entry_point::ARG_NAME)
            .display_order(DisplayOrder::SessionPackageName as usize)
    }

    pub fn get(matches: &ArgMatches) -> Option<&str> {
        matches.get_one::<String>(ARG_NAME).map(String::as_str)
    }
}

pub(super) mod session_entry_point {
    use super::*;

    pub const ARG_NAME: &str = "session-entry-point";
    const ARG_VALUE_NAME: &str = "NAME";
    const ARG_HELP: &str = "Name of the method that will be used when calling the session contract";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(false)
            .display_order(DisplayOrder::SessionEntryPoint as usize)
    }

    pub fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default()
    }
}

pub(super) mod session_version {
    use super::*;

    pub const ARG_NAME: &str = "session-version";
    const ARG_VALUE_NAME: &str = common::ARG_INTEGER;
    const ARG_HELP: &str = "Version of the called session contract. Latest will be used by default";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(false)
            .display_order(DisplayOrder::SessionVersion as usize)
    }

    pub fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default()
    }
}

pub(super) mod payment_hash {
    use super::*;

    pub const ARG_NAME: &str = "payment-hash";
    const ARG_VALUE_NAME: &str = common::ARG_HEX_STRING;
    const ARG_HELP: &str = "Hex-encoded hash of the stored contract to be called as the payment";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(false)
            .requires(payment_entry_point::ARG_NAME)
            .display_order(DisplayOrder::PaymentHash as usize)
    }

    pub fn get(matches: &ArgMatches) -> Option<&str> {
        matches.get_one::<String>(ARG_NAME).map(String::as_str)
    }
}

pub(super) mod payment_name {
    use super::*;

    pub const ARG_NAME: &str = "payment-name";
    const ARG_VALUE_NAME: &str = "NAME";
    const ARG_HELP: &str = "Name of the stored contract (associated with the executing account) \
    to be called as the payment";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(false)
            .requires(payment_entry_point::ARG_NAME)
            .display_order(DisplayOrder::PaymentName as usize)
    }

    pub fn get(matches: &ArgMatches) -> Option<&str> {
        matches.get_one::<String>(ARG_NAME).map(String::as_str)
    }
}

pub(super) mod payment_package_hash {
    use super::*;

    pub const ARG_NAME: &str = "payment-package-hash";
    const ARG_VALUE_NAME: &str = common::ARG_HEX_STRING;
    const ARG_HELP: &str = "Hex-encoded hash of the stored package to be called as the payment";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(false)
            .requires(payment_entry_point::ARG_NAME)
            .display_order(DisplayOrder::PaymentPackageHash as usize)
    }

    pub fn get(matches: &ArgMatches) -> Option<&str> {
        matches.get_one::<String>(ARG_NAME).map(String::as_str)
    }
}

pub(super) mod payment_package_name {
    use super::*;

    pub const ARG_NAME: &str = "payment-package-name";
    const ARG_VALUE_NAME: &str = "NAME";
    const ARG_HELP: &str = "Name of the stored package to be called as the payment";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(false)
            .requires(payment_entry_point::ARG_NAME)
            .display_order(DisplayOrder::PaymentPackageName as usize)
    }

    pub fn get(matches: &ArgMatches) -> Option<&str> {
        matches.get_one::<String>(ARG_NAME).map(String::as_str)
    }
}

pub(super) mod payment_entry_point {
    use super::*;

    pub const ARG_NAME: &str = "payment-entry-point";
    const ARG_VALUE_NAME: &str = "NAME";
    const ARG_HELP: &str = "Name of the method that will be used when calling the payment contract";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(false)
            .display_order(DisplayOrder::PaymentEntryPoint as usize)
    }

    pub fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default()
    }
}

pub(super) mod payment_version {
    use super::*;

    pub const ARG_NAME: &str = "payment-version";
    const ARG_VALUE_NAME: &str = common::ARG_INTEGER;
    const ARG_HELP: &str = "Version of the called payment contract. Latest will be used by default";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(false)
            .display_order(DisplayOrder::PaymentVersion as usize)
    }

    pub fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default()
    }
}
