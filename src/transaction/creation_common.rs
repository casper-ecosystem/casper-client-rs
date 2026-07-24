//! This module contains structs and helpers which are used by multiple subcommands related to
//! creating transactions.

use std::process;
use std::str::FromStr;

use clap::{Arg, ArgAction, ArgGroup, ArgMatches, Command};

use casper_client::cli::{json_args_help, simple_args_help, CliError, TransactionStrParams};
use casper_types::TransactionRuntimeParams;
use transaction_runtime::TransactionRuntime;

use crate::common;

const SESSION_ARG_GROUP: &str = "session-args";

const INITIATOR_ARG_GROUP: &str = "initiator";

/// This struct defines the order in which the args are shown for this subcommand's help message.
pub(super) enum DisplayOrder {
    ShowSimpleArgExamples,
    ShowJsonArgExamples,
    NodeAddress,
    SecretKey,
    SpeculativeExec,
    TransactionPath,
    Output,
    Force,
    Target,
    TransferAmount,
    TransferId,
    Timestamp,
    Ttl,
    TransferredValue,
    ChainName,
    MaximumDelegationRate,
    MinimumDelegationRate,
    ReservedSlots,
    Reservations,
    Delegators,
    Source,
    SessionArgSimple,
    SessionArgsJson,
    ChunkedArgs,
    SessionEntryPoint,
    SessionVersion,
    PublicKey,
    NewPublicKey,
    PackageAlias,
    PackageAddr,
    ContractPackageHash,
    EntityAlias,
    PaymentAmount,
    PricingMode,
    StandardPayment,
    Receipt,
    GasPriceTolerance,
    AdditionalComputationFactor,
    IsInstallUpgrade,
    TransactionAmount,
    Validator,
    NewValidator,
    Delegator,
    EntityAddr,
    ContractHash,
    MinBidOverride,
    MajorVersion,
    TransactionRuntime,
    RpcId,
    Verbose,
}

/// Handles providing the arg for and executing the show-simple-arg-examples option.
pub(super) mod show_simple_arg_examples {
    use super::*;

    pub(in crate::transaction) const ARG_NAME: &str = "show-simple-arg-examples";
    const ARG_ALIAS: &str = "show-arg-examples";
    const ARG_SHORT: char = 'e';
    const ARG_HELP: &str =
        "If passed, all other options are ignored and a set of examples of session-/payment-args \
        is printed";

    pub(in crate::transaction) fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .alias(ARG_ALIAS)
            .long(ARG_NAME)
            .short(ARG_SHORT)
            .required(false)
            .action(ArgAction::SetTrue)
            .help(ARG_HELP)
            .display_order(DisplayOrder::ShowSimpleArgExamples as usize)
    }

    pub(in crate::transaction) fn get(matches: &ArgMatches) -> bool {
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

    pub(in crate::transaction) const ARG_NAME: &str = "show-json-args-examples";
    const ARG_SHORT: char = 'j';
    const ARG_HELP: &str = "If passed, all other options are ignored and a set of examples of \
        session-/payment-args-json is printed";

    pub(in crate::transaction) fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .short(ARG_SHORT)
            .required(false)
            .action(ArgAction::SetTrue)
            .help(ARG_HELP)
            .display_order(DisplayOrder::ShowJsonArgExamples as usize)
    }

    pub(in crate::transaction) fn get(matches: &ArgMatches) -> bool {
        if let Some(true) = matches.get_one::<bool>(ARG_NAME) {
            println!("Examples for passing values via --session-args-json or --payment-args-json:");
            println!();
            println!("{}", json_args_help::info_and_examples());
            return true;
        }

        false
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

    pub(in crate::transaction) fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::Timestamp as usize)
    }

    pub(in crate::transaction) fn get(matches: &ArgMatches) -> &str {
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
        "Time that the transaction will remain valid for. A transaction can only be included in a block \
        between `timestamp` and `timestamp + ttl`. Input examples: '1hr 12min', '30min 50sec', \
        '1day'. For all options, see \
        https://docs.rs/humantime/latest/humantime/fn.parse_duration.html";

    pub(in crate::transaction) fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .default_value(ARG_DEFAULT)
            .help(ARG_HELP)
            .display_order(DisplayOrder::Ttl as usize)
    }

    pub(in crate::transaction) fn get(matches: &ArgMatches) -> &str {
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
        "Name of the chain, to avoid the transaction from being accidentally or maliciously included in \
        a different chain";

    pub(in crate::transaction) fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .required(true)
            .help(ARG_HELP)
            .display_order(DisplayOrder::ChainName as usize)
    }

    pub(in crate::transaction) fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_else(|| panic!("should have {} arg", ARG_NAME))
    }
}

/// Handles providing the arg for and retrieval of simple session args.
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

    pub(in crate::transaction) mod session {
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

/// Handles providing the arg for and retrieval of JSON session args.
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

    pub(in crate::transaction) mod session {
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

/// Handles providing the arg for and retrieval of chunked arguments passed as base16 string.
pub(super) mod chunked_args {
    use super::*;
    use once_cell::sync::Lazy;

    pub const ARG_NAME: &str = "chunked-args";

    const ARG_VALUE_NAME: &str = "BYTES";

    static ARG_HELP: Lazy<String> = Lazy::new(|| {
        format!(
            "Chunked arg bytes as base16 '--{}'.",
            show_json_args_examples::ARG_NAME,
        )
    });

    pub fn get(matches: &ArgMatches) -> Option<Vec<u8>> {
        matches
            .get_one::<String>(ARG_NAME)
            .and_then(|data| base16::decode(data).ok())
    }

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP.as_str())
            .display_order(DisplayOrder::ChunkedArgs as usize)
    }
}

pub(super) mod payment_amount {
    use super::*;
    pub(in crate::transaction) const ARG_NAME: &str = "payment-amount";

    const ARG_VALUE_NAME: &str = common::ARG_INTEGER;

    const ARG_SHORT: char = 'p';
    const ARG_HELP: &str =
        "Uses the standard-payment system contract. The value is the amount arg \
                            of the standard-payment contract";

    pub(in crate::transaction) fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .short(ARG_SHORT)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::PaymentAmount as usize)
    }

    pub fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default()
    }
}

pub(super) mod receipt {
    use super::*;
    pub(in crate::transaction) const ARG_NAME: &str = "receipt";

    const ARG_VALUE_NAME: &str = common::ARG_HEX_STRING;
    const ARG_HELP: &str = "The digest representing the a previous reservation of funds to pay for the current transaction.";

    pub(in crate::transaction) fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::Receipt as usize)
    }

    pub fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default()
    }
}

pub(super) mod standard_payment {
    use super::*;
    pub(in crate::transaction) const ARG_NAME: &str = "standard-payment";

    const ARG_VALUE_NAME: &str = common::ARG_STRING;

    const ARG_HELP: &str = "Flag to determine if this transaction uses standard or custom payment.";

    pub(in crate::transaction) fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::StandardPayment as usize)
    }

    pub fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default()
    }
}

pub(super) mod gas_price_tolerance {
    use super::*;
    pub(in crate::transaction) const ARG_NAME: &str = "gas-price-tolerance";

    const ARG_VALUE_NAME: &str = common::ARG_INTEGER;

    const ARG_ALIAS: &str = "gas-price";
    const ARG_SHORT: char = 'g';
    const ARG_HELP: &str =
        "The maximum gas price that the user is willing to pay for the transaction";

    pub(in crate::transaction) fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .alias(ARG_ALIAS)
            .short(ARG_SHORT)
            .required(true)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::GasPriceTolerance as usize)
    }

    pub fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default()
    }
}

pub(super) mod transfer_amount {
    use super::*;
    pub(in crate::transaction) const ARG_NAME: &str = "transfer-amount";

    const ARG_VALUE_NAME: &str = common::ARG_INTEGER;

    const ARG_HELP: &str = "Uses the transfer system contract. The value is the amount arg \
                            of the transfer contract";

    pub(in crate::transaction) fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .required(true)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::TransferAmount as usize)
    }

    pub fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default()
    }
}

pub(super) mod pricing_mode {
    use super::*;
    use clap::{builder::PossibleValue, value_parser, ValueEnum};
    use std::str::FromStr;

    pub(in crate::transaction) const ARG_NAME: &str = "pricing-mode";

    const ARG_VALUE_NAME: &str = "classic|reserved|fixed";
    const ARG_HELP: &str = "Used to identify the payment mode chosen to execute the transaction";
    const ARG_DEFAULT: &str = PricingMode::CLASSIC;

    pub(in crate::transaction) fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .default_value(ARG_DEFAULT)
            .help(ARG_HELP)
            .display_order(DisplayOrder::PricingMode as usize)
            .value_parser(value_parser!(PricingMode))
    }

    #[derive(Debug, Clone, Copy)]
    pub enum PricingMode {
        Classic,
        Reserved,
        Fixed,
    }

    impl PricingMode {
        const CLASSIC: &'static str = "classic";
        const RESERVED: &'static str = "reserved";
        const FIXED: &'static str = "fixed";

        pub(crate) fn as_str(&self) -> &str {
            match self {
                Self::Classic => Self::CLASSIC,
                Self::Reserved => Self::RESERVED,
                Self::Fixed => Self::FIXED,
            }
        }
    }

    impl ValueEnum for PricingMode {
        fn value_variants<'a>() -> &'a [Self] {
            &[Self::Classic, Self::Reserved, Self::Fixed]
        }

        fn to_possible_value(&self) -> Option<PossibleValue> {
            Some(match self {
                Self::Classic => PossibleValue::new(PricingMode::CLASSIC),
                Self::Reserved => PossibleValue::new(PricingMode::RESERVED),
                Self::Fixed => PossibleValue::new(PricingMode::FIXED),
            })
        }
    }

    impl FromStr for PricingMode {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().as_str() {
                PricingMode::CLASSIC => Ok(Self::Classic),
                PricingMode::RESERVED => Ok(Self::Reserved),
                PricingMode::FIXED => Ok(Self::Fixed),
                _ => Err(format!("'{}' is not a valid pricing option", s)),
            }
        }
    }

    pub fn get(matches: &ArgMatches) -> Option<&PricingMode> {
        matches.get_one(ARG_NAME)
    }
}

pub(super) mod additional_computation_factor {
    use super::*;
    pub(in crate::transaction) const ARG_NAME: &str = "additional-computation-factor";

    const ARG_VALUE_NAME: &str = common::ARG_INTEGER;

    const ARG_ALIAS: &str = "additional-computation";
    const ARG_SHORT: char = 'c';
    const ARG_HELP: &str =
        "User-specified additional computation factor for \"fixed\" pricing_mode";
    const ARG_DEFAULT: &str = "0";

    pub(in crate::transaction) fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .alias(ARG_ALIAS)
            .short(ARG_SHORT)
            .required(false)
            .default_value(ARG_DEFAULT)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::AdditionalComputationFactor as usize)
    }

    pub fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default()
    }
}

pub(super) mod transaction_runtime {
    use super::*;
    use clap::{builder::PossibleValue, value_parser, ValueEnum};
    use std::str::FromStr;

    pub(in crate::transaction) const ARG_NAME: &str = "transaction-runtime";

    const ARG_VALUE_NAME: &str = "vm-casper-v1|vm-casper-v2";
    const ARG_HELP: &str = "Transaction runtime";
    const ARG_DEFAULT: &str = TransactionRuntime::VM_CASPER_V1;

    pub(in crate::transaction) fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .default_value(ARG_DEFAULT)
            .help(ARG_HELP)
            .display_order(DisplayOrder::TransactionRuntime as usize)
            .value_parser(value_parser!(TransactionRuntime))
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub enum TransactionRuntime {
        #[default]
        VmCasperV1,
        VmCasperV2,
    }

    impl From<TransactionRuntime> for casper_types::ContractRuntimeTag {
        fn from(runtime: TransactionRuntime) -> Self {
            match runtime {
                TransactionRuntime::VmCasperV1 => casper_types::ContractRuntimeTag::VmCasperV1,
                TransactionRuntime::VmCasperV2 => casper_types::ContractRuntimeTag::VmCasperV2,
            }
        }
    }

    impl TransactionRuntime {
        const VM_CASPER_V1: &'static str = "vm-casper-v1";
        const VM_CASPER_V2: &'static str = "vm-casper-v2";
    }

    impl ValueEnum for TransactionRuntime {
        fn value_variants<'a>() -> &'a [Self] {
            &[Self::VmCasperV1, Self::VmCasperV2]
        }

        fn to_possible_value(&self) -> Option<PossibleValue> {
            Some(match self {
                Self::VmCasperV1 => PossibleValue::new(TransactionRuntime::VM_CASPER_V1),
                Self::VmCasperV2 => PossibleValue::new(TransactionRuntime::VM_CASPER_V2),
            })
        }
    }

    impl FromStr for TransactionRuntime {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().as_str() {
                Self::VM_CASPER_V1 => Ok(Self::VmCasperV1),
                Self::VM_CASPER_V2 => Ok(Self::VmCasperV2),
                _ => Err(format!("'{}' is not a valid transaction runtime", s)),
            }
        }
    }

    pub fn get(matches: &ArgMatches) -> Option<&TransactionRuntime> {
        matches.get_one(ARG_NAME)
    }
}

pub(super) mod initiator_address {
    use super::*;
    pub(in crate::transaction) const ARG_NAME: &str = "initiator-address";

    const ARG_VALUE_NAME: &str = common::ARG_HEX_STRING;
    const ARG_HELP: &str = "Used to specify the account initiating the transaction. This can be a \
                            a public key, account hash, or an entity address. \
    ";

    pub(in crate::transaction) fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::PricingMode as usize)
    }

    pub fn get(matches: &ArgMatches) -> String {
        let value = matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default();
        common::public_key::try_read_from_file(value)
            .unwrap_or_else(|_| panic!("should have {} arg", ARG_NAME))
    }
}

pub(super) fn apply_common_creation_options(
    mut subcommand: Command,
    require_secret_key: bool,
    include_node_address: bool,
    include_transaction_args: bool,
) -> Command {
    if include_node_address {
        subcommand = subcommand.arg(common::node_address::arg(
            DisplayOrder::NodeAddress as usize,
        ));
    }

    let secret_key_arg = if require_secret_key {
        common::secret_key::arg(DisplayOrder::SecretKey as usize, "")
            .required_unless_present(show_simple_arg_examples::ARG_NAME)
            .required_unless_present(show_json_args_examples::ARG_NAME)
    } else {
        common::secret_key::arg(
            DisplayOrder::SecretKey as usize,
            ". If not provided, the transaction will not be signed and will remain invalid until \
            signed, for example by running the `sign-transaction` subcommand.",
        )
    };

    if include_transaction_args {
        subcommand = subcommand
            .arg(arg_simple::session::arg())
            .arg(args_json::session::arg())
            .arg(show_simple_arg_examples::arg())
            .arg(show_json_args_examples::arg())
            // Group the session-arg args so only one style is used to ensure consistent ordering.
            .group(
                ArgGroup::new(SESSION_ARG_GROUP)
                    .arg(arg_simple::session::ARG_NAME)
                    .arg(args_json::session::ARG_NAME)
                    .required(false),
            )
            .group(
                ArgGroup::new("session")
                    .arg(show_simple_arg_examples::ARG_NAME)
                    .arg(show_json_args_examples::ARG_NAME)
                    .required(false),
            )
            .group(
                // This group duplicates all the args in the "session" and "show-examples" groups, but
                // ensures at least one of them are provided.
                ArgGroup::new("session-and-show-examples")
                    .arg(show_simple_arg_examples::ARG_NAME)
                    .arg(show_json_args_examples::ARG_NAME)
                    .multiple(true)
                    .required(false),
            );
    }

    subcommand = subcommand
        .arg(secret_key_arg)
        .arg(initiator_address::arg())
        .group(
            ArgGroup::new(INITIATOR_ARG_GROUP)
                .arg(common::secret_key::ARG_NAME)
                .arg(initiator_address::ARG_NAME)
                .required(true),
        )
        .arg(timestamp::arg())
        .arg(ttl::arg())
        .arg(chain_name::arg())
        .arg(output::arg())
        .arg(payment_amount::arg())
        .arg(pricing_mode::arg())
        .arg(additional_computation_factor::arg())
        .arg(gas_price_tolerance::arg())
        .arg(receipt::arg())
        .arg(standard_payment::arg())
        .group(
            ArgGroup::new("Classic payment")
                .arg(payment_amount::ARG_NAME)
                .arg(gas_price_tolerance::ARG_NAME)
                .multiple(true)
                .required(false),
        )
        .group(
            ArgGroup::new("Reserved payment")
                .arg(receipt::ARG_NAME)
                .required(false),
        )
        .group(
            ArgGroup::new("Fixed Payment")
                .arg(gas_price_tolerance::ARG_NAME)
                .required(false),
        );
    subcommand
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
        "Path to output transaction file. If omitted, defaults to stdout. If the file already exists, \
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

pub(super) mod wasm_path {
    use super::*;

    const ARG_NAME: &str = "wasm-path";
    const ARG_SHORT: char = 'w';
    const ARG_VALUE_NAME: &str = common::ARG_PATH;
    const ARG_HELP: &str = "Path to compiled Wasm session code";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .required(true)
            .long(ARG_NAME)
            .short(ARG_SHORT)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::TransactionPath as usize)
    }

    pub fn get(matches: &ArgMatches) -> Option<&str> {
        matches.get_one::<String>(ARG_NAME).map(String::as_str)
    }
}

pub(super) mod is_install_upgrade {
    use super::*;

    const ARG_NAME: &str = "install-upgrade";
    const ARG_HELP: &str = "Flag to indicate if the Wasm is an install/upgrade";

    pub fn arg(order: usize) -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .required(false)
            .action(ArgAction::SetTrue)
            .help(ARG_HELP)
            .display_order(order)
    }

    pub fn get(matches: &ArgMatches) -> bool {
        matches.get_flag(ARG_NAME)
    }
}

pub(super) mod transferred_value {
    use super::*;

    const ARG_NAME: &str = "transferred-value";
    const ARG_SHORT: char = 'T';
    const ARG_VALUE_NAME: &str = "integer";
    const ARG_HELP: &str = "Transferred token value";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .required(false)
            .long(ARG_NAME)
            .short(ARG_SHORT)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::TransferredValue as usize)
    }

    pub(super) fn get_raw(matches: &ArgMatches) -> Option<&String> {
        matches.get_one(ARG_NAME)
    }

    pub(super) fn get(matches: &ArgMatches) -> Result<Option<u64>, CliError> {
        let maybe_result = matches.get_one::<String>(ARG_NAME).map(|str| {
            str.parse::<u64>()
                .map_err(|err| CliError::FailedToParseInt {
                    context: "Transferred value",
                    error: err,
                })
        });
        match maybe_result {
            Some(res) => res.map(Some),
            None => Ok(None),
        }
    }
}

pub(super) mod public_key {
    use super::*;
    use casper_client::cli::CliError;
    use casper_types::{crypto, AsymmetricType, PublicKey};

    pub const ARG_NAME: &str = "public-key";
    const ARG_VALUE_NAME: &str = "FORMATTED STRING or PATH";
    const ARG_HELP: &str =
        "The hex-encoded public key of the account context under which the session code will be \
        executed. This must be a properly formatted public key. The public key may instead be read \
        in from a file, in which case enter the path to the file as the --public-key \
        argument. The file should be one of the two public key files generated via the `keygen` \
        subcommand; \"public_key_hex\" or \"public_key.pem\".";

    pub fn arg(order: usize) -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .required(true)
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

    pub(crate) fn parse_public_key(value: &str) -> Result<PublicKey, CliError> {
        let public_key =
            PublicKey::from_hex(value).map_err(|error| casper_client::Error::CryptoError {
                context: "session account",
                error: crypto::ErrorExt::from(error),
            })?;
        Ok(public_key)
    }
}

pub(super) mod new_public_key {
    use super::*;
    use casper_client::cli::CliError;
    use casper_types::{crypto, AsymmetricType, PublicKey};

    pub const ARG_NAME: &str = "new-public-key";
    const ARG_VALUE_NAME: &str = "FORMATTED STRING or PATH";
    const ARG_HELP: &str =
        "The hex-encoded public key of the account that the validator bid will be transferred to. \
        This must be a properly formatted public key. The public key may instead be read \
        in from a file, in which case enter the path to the file as the --new-public-key \
        argument.";

    pub fn arg(order: usize) -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .required(true)
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

    pub(super) fn parse_public_key(value: &str) -> Result<PublicKey, CliError> {
        let public_key =
            PublicKey::from_hex(value).map_err(|error| casper_client::Error::CryptoError {
                context: "new public key",
                error: crypto::ErrorExt::from(error),
            })?;
        Ok(public_key)
    }
}

pub(super) mod min_bid_override {
    use super::*;

    const ARG_NAME: &str = "min-bid-override";

    const ARG_HELP: &str = "Flag to override the min bid staking amount check";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .required(false)
            .action(ArgAction::SetTrue)
            .help(ARG_HELP)
            .display_order(DisplayOrder::MinBidOverride as usize)
    }

    pub fn get(matches: &ArgMatches) -> bool {
        matches.get_flag(ARG_NAME)
    }
}

pub(super) mod entity_addr {
    use super::*;
    use casper_client::cli::CliError;
    use casper_client::Error;
    use casper_types::{EntityAddr, Key};

    pub const ARG_NAME: &str = "entity-address";
    const ARG_VALUE_NAME: &str = "FORMATTED STRING";
    const ARG_HELP: &str = "The formatted string representing an addressable entity address.";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::EntityAddr as usize)
    }

    pub fn get(matches: &ArgMatches) -> Result<String, CliError> {
        let value = matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default();
        common::public_key::try_read_from_file(value)
    }

    pub(super) fn parse_entity_addr(value: String) -> Result<EntityAddr, CliError> {
        let entity_addr =
            Key::from_formatted_str(&value).map_err(|error| CliError::FailedToParseKey {
                context: "entity address",
                error,
            })?;
        match entity_addr {
            Key::AddressableEntity(entity_addr) => Ok(entity_addr),
            _ => Err(CliError::from(Error::InvalidKeyVariant {
                expected_variant: "AddressableEntity".to_string(),
                actual: entity_addr,
            })),
        }
    }
}

pub(super) mod contract_hash {
    use super::*;
    use casper_client::cli::CliError;
    use casper_client::Error;
    use casper_types::{EntityAddr, Key};

    pub const ARG_NAME: &str = "contract-hash";
    const ARG_VALUE_NAME: &str = "FORMATTED STRING";
    const ARG_HELP: &str = "The formatted string prefixed by hash- representing a 1.x contract.";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::ContractHash as usize)
    }

    pub fn get(matches: &ArgMatches) -> Option<&str> {
        matches.get_one::<String>(ARG_NAME).map(String::as_str)
    }

    pub(super) fn parse_contract_hash(value: &str) -> Result<EntityAddr, CliError> {
        let entity_addr =
            Key::from_formatted_str(value).map_err(|error| CliError::FailedToParseKey {
                context: "contract hash",
                error,
            })?;
        match entity_addr {
            Key::Hash(hash_addr) => Ok(EntityAddr::SmartContract(hash_addr)),
            _ => Err(CliError::from(Error::InvalidKeyVariant {
                expected_variant: "Key::Hash".to_string(),
                actual: entity_addr,
            })),
        }
    }
}

pub(super) mod package_addr {
    use super::*;
    use casper_client::{cli::CliError, Error};
    use casper_types::{Key, PackageAddr};

    pub const ARG_NAME: &str = "package-address";
    const ARG_VALUE_NAME: &str = "FORMATTED STRING or PATH";
    const ARG_HELP: &str = "The formatted string representing an package address.";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::ContractPackageHash as usize)
    }

    pub fn get(matches: &ArgMatches) -> Option<&str> {
        matches.get_one::<String>(ARG_NAME).map(String::as_str)
    }

    pub(super) fn parse_package_addr(value: Option<&str>) -> Result<PackageAddr, CliError> {
        match value {
            None => Err(CliError::FailedToParsePackageAddr),
            Some(value) => {
                let package_addr =
                    Key::from_formatted_str(value).map_err(|error| CliError::FailedToParseKey {
                        context: "package address",
                        error,
                    })?;
                match package_addr {
                    Key::SmartContract(package_addr) => Ok(package_addr),
                    _ => Err(CliError::Core(Error::InvalidKeyVariant {
                        expected_variant: "Package Address".to_string(),
                        actual: package_addr,
                    })),
                }
            }
        }
    }
}

pub(super) mod contract_package_hash {
    use super::*;
    use casper_client::{cli::CliError, Error};
    use casper_types::{Key, PackageAddr};

    pub const ARG_NAME: &str = "contract-package-hash";
    const ARG_VALUE_NAME: &str = "FORMATTED STRING";
    const ARG_HELP: &str =
        "The formatted string prefixed by hash- representing a 1.x contract package";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::PackageAddr as usize)
    }

    pub fn get(matches: &ArgMatches) -> Option<&str> {
        matches.get_one::<String>(ARG_NAME).map(String::as_str)
    }

    pub(super) fn parse_package_addr(value: &str) -> Result<PackageAddr, CliError> {
        let package_addr =
            Key::from_formatted_str(value).map_err(|error| CliError::FailedToParseKey {
                context: "contract-package-hash",
                error,
            })?;
        match package_addr {
            Key::Hash(package_addr) => Ok(package_addr),
            _ => Err(CliError::Core(Error::InvalidKeyVariant {
                expected_variant: "ContractPackageHash".to_string(),
                actual: package_addr,
            })),
        }
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

    pub fn get(matches: &ArgMatches) -> Option<&str> {
        matches.get_one::<String>(ARG_NAME).map(String::as_str)
    }
}

fn parse_arg_to_int<T: FromStr<Err = std::num::ParseIntError>>(
    value: &str,
    context: &'static str,
) -> Result<T, CliError> {
    value
        .parse()
        .map_err(move |err| CliError::FailedToParseInt {
            context,
            error: err,
        })
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

    pub fn get(matches: &ArgMatches) -> Option<u32> {
        match matches.get_one::<String>(ARG_NAME) {
            Some(arg) => parse_arg_to_int(arg, "session-version").ok(),
            None => None,
        }
    }
}

pub(super) mod major_version {
    use super::*;

    pub const ARG_NAME: &str = "major-version";
    const ARG_VALUE_NAME: &str = common::ARG_INTEGER;
    const ARG_HELP: &str =
        "The major version of the called session contract. Required if specifying a version";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(false)
            .display_order(DisplayOrder::MajorVersion as usize)
    }

    pub fn get(matches: &ArgMatches) -> Option<u32> {
        match matches.get_one::<String>(ARG_NAME) {
            Some(arg) => parse_arg_to_int(arg, "major-version").ok(),
            None => None,
        }
    }
}

mod package_name_arg {
    use super::*;

    pub const ARG_NAME: &str = "transaction-package-name";
    pub const ARG_ALIAS: &str = "txn-package-name";
    const ARG_VALUE_NAME: &str = common::ARG_STRING;
    const ARG_HELP: &str = "The name of a stored transaction package.";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .alias(ARG_ALIAS)
            .required(false)
            .display_order(DisplayOrder::PackageAlias as usize)
    }

    pub fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default()
    }
}

mod entity_alias_arg {
    use super::*;

    pub const ARG_NAME: &str = "entity-alias";
    const ARG_VALUE_NAME: &str = common::ARG_STRING;
    const ARG_HELP: &str = "The alias for targeting a stored entity.";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(true)
            .display_order(DisplayOrder::EntityAlias as usize)
    }

    pub fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default()
    }
}

mod delegation_rate {
    use super::*;
    use casper_client::cli::CliError;
    pub const ARG_NAME: &str = "delegation-rate";
    const ARG_VALUE_NAME: &str = common::ARG_INTEGER;
    const ARG_HELP: &str = "the delegation rate for the add-bid transaction";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(true)
            .display_order(DisplayOrder::MinimumDelegationRate as usize)
    }

    pub fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default()
    }

    pub(super) fn parse_delegation_rate(value: &str) -> Result<u8, CliError> {
        value
            .parse::<u8>()
            .map_err(|err| CliError::FailedToParseInt {
                context: "Add Bid",
                error: err,
            })
    }
}

mod minimum_delegation_amount {
    use super::*;
    use casper_client::cli::CliError;
    pub const ARG_NAME: &str = "minimum-delegation-amount";
    const ALIAS: &str = "min-amount";
    const ARG_VALUE_NAME: &str = common::ARG_INTEGER;
    const ARG_HELP: &str = "the minimum delegation amount for the add-bid transaction";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .alias(ALIAS)
            .help(ARG_HELP)
            .required(false)
            .display_order(DisplayOrder::MinimumDelegationRate as usize)
    }

    pub fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default()
    }

    pub(super) fn parse_delegation_amount(value: &str) -> Result<Option<u64>, CliError> {
        let delegation_amount = if value.is_empty() {
            None
        } else {
            Some(
                value
                    .parse::<u64>()
                    .map_err(|err| CliError::FailedToParseInt {
                        context: "Add Bid: Minimum delegation amount",
                        error: err,
                    })?,
            )
        };
        Ok(delegation_amount)
    }
}

mod maximum_delegation_amount {
    use super::*;
    use casper_client::cli::CliError;
    pub const ARG_NAME: &str = "maximum-delegation-amount";
    const ALIAS: &str = "max-amount";
    const ARG_VALUE_NAME: &str = common::ARG_INTEGER;
    const ARG_HELP: &str = "the maximum delegation amount for the add-bid transaction";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .alias(ALIAS)
            .help(ARG_HELP)
            .required(false)
            .display_order(DisplayOrder::MaximumDelegationRate as usize)
    }

    pub fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default()
    }

    pub(super) fn parse_delegation_amount(value: &str) -> Result<Option<u64>, CliError> {
        let delegation_amount = if value.is_empty() {
            None
        } else {
            Some(
                value
                    .parse::<u64>()
                    .map_err(|err| CliError::FailedToParseInt {
                        context: "Add Bid: Maximum delegation amount",
                        error: err,
                    })?,
            )
        };
        Ok(delegation_amount)
    }
}

mod reserved_slots {
    use super::*;
    use casper_client::cli::CliError;
    pub const ARG_NAME: &str = "reserved-slots";
    const ARG_VALUE_NAME: &str = common::ARG_INTEGER;
    const ARG_HELP: &str = "number of reserved delegator slots for the add-bid transaction";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(false)
            .display_order(DisplayOrder::ReservedSlots as usize)
    }

    pub fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default()
    }

    pub(super) fn parse_reserved_slots(value: &str) -> Result<Option<u32>, CliError> {
        let delegation_amount = if value.is_empty() {
            None
        } else {
            Some(
                value
                    .parse::<u32>()
                    .map_err(|err| CliError::FailedToParseInt {
                        context: "Add Bid: Reserved slots",
                        error: err,
                    })?,
            )
        };
        Ok(delegation_amount)
    }
}

mod validator {
    use super::*;
    pub const ARG_NAME: &str = "validator";
    const ARG_VALUE_NAME: &str = common::ARG_STRING;
    const ARG_HELP: &str = "the validator's public key (as a formatted string)";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(true)
            .display_order(DisplayOrder::Validator as usize)
    }

    pub fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default()
    }
}

mod new_validator {
    use super::*;
    pub const ARG_NAME: &str = "new-validator";
    const ARG_VALUE_NAME: &str = common::ARG_STRING;
    const ARG_HELP: &str = "the validator for the delegate transaction";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(true)
            .display_order(DisplayOrder::NewValidator as usize)
    }

    pub fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default()
    }
}

mod delegator {
    use super::*;
    pub const ARG_NAME: &str = "delegator";
    const ARG_VALUE_NAME: &str = common::ARG_STRING;
    const ARG_HELP: &str =
        "the delegators public key (as a formatted string) for the delegate transaction";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(true)
            .display_order(DisplayOrder::Delegator as usize)
    }

    pub fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default()
    }
}

mod transaction_amount {
    use super::*;
    use casper_client::cli::CliError;
    use casper_types::U512;

    pub const ARG_NAME: &str = "transaction-amount";
    const ARG_VALUE_NAME: &str = common::ARG_INTEGER;
    const ARG_HELP: &str = "the amount of CSPR motes for the transaction";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(true)
            .display_order(DisplayOrder::TransactionAmount as usize)
    }

    pub fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default()
    }

    pub(super) fn parse_transaction_amount(value: &str) -> Result<U512, CliError> {
        if !value.is_empty() {
            U512::from_dec_str(value).map_err(|_| {
                CliError::InvalidCLValue("Failed to parse U512 for add-bid".to_string())
            })
        } else {
            Err(CliError::InvalidArgument {
                context: "parse_transaction_amount",
                error: "Transaction amount cannot be empty".to_string(),
            })
        }
    }
}

mod reservations {
    use super::*;
    use casper_client::cli::CliError;
    use casper_types::system::auction::Reservation;

    pub const ARG_NAME: &str = "reservations";
    const ARG_VALUE_NAME: &str = "JSON serialized Vec<Reservation>";
    const ARG_HELP: &str = "list of reservations to add for the add-reservations transaction";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(true)
            .display_order(DisplayOrder::Reservations as usize)
    }

    pub fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default()
    }

    pub(super) fn parse_reservations(value: &str) -> Result<Vec<Reservation>, CliError> {
        if !value.is_empty() {
            serde_json::from_str(value).map_err(|_| {
                CliError::InvalidCLValue(
                    "Failed to parse reservations for add-reservations".to_string(),
                )
            })
        } else {
            Err(CliError::InvalidArgument {
                context: "parse_reservations",
                error: "Reservations cannot be empty".to_string(),
            })
        }
    }
}

mod delegators {
    use super::*;
    use casper_client::cli::CliError;
    use casper_types::system::auction::DelegatorKind;

    pub const ARG_NAME: &str = "delegators";
    const ARG_VALUE_NAME: &str = "JSON serialized Vec<DelegatorKind>";
    const ARG_HELP: &str = "list of delegator public keys for the cancel-reservations transaction";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(true)
            .display_order(DisplayOrder::Delegators as usize)
    }

    pub fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_default()
    }

    pub(super) fn parse_delegators(value: &str) -> Result<Vec<DelegatorKind>, CliError> {
        if !value.is_empty() {
            serde_json::from_str(value).map_err(|_| {
                CliError::InvalidCLValue(
                    "Failed to parse delegators for cancel-reservations".to_string(),
                )
            })
        } else {
            Err(CliError::InvalidArgument {
                context: "parse_delegators",
                error: "Delegators cannot be empty".to_string(),
            })
        }
    }
}

pub(super) mod add_bid {
    use super::*;
    use casper_client::cli::{CliError, TransactionBuilderParams, TransactionStrParams};

    pub const NAME: &str = "add-bid";

    const ACCEPT_SESSION_ARGS: bool = false;

    const ABOUT: &str = "Creates a new add-bid transaction";
    pub fn build() -> Command {
        apply_common_creation_options(
            add_args(Command::new(NAME).about(ABOUT)),
            false,
            false,
            ACCEPT_SESSION_ARGS,
        )
    }

    pub fn put_transaction_build() -> Command {
        add_rpc_args(build())
    }

    pub fn run(
        matches: &ArgMatches,
    ) -> Result<(TransactionBuilderParams, TransactionStrParams), CliError> {
        let public_key_str = public_key::get(matches)?;
        let public_key = public_key::parse_public_key(&public_key_str)?;

        let delegation_rate_str = delegation_rate::get(matches);
        let delegation_rate = delegation_rate::parse_delegation_rate(delegation_rate_str)?;

        let amount_str = transaction_amount::get(matches);
        let amount = transaction_amount::parse_transaction_amount(amount_str)?;

        let minimum_amount_string = minimum_delegation_amount::get(matches);
        let minimum_delegation_amount =
            minimum_delegation_amount::parse_delegation_amount(minimum_amount_string)?;

        let maximum_amount_string = maximum_delegation_amount::get(matches);
        let maximum_delegation_amount =
            maximum_delegation_amount::parse_delegation_amount(maximum_amount_string)?;

        let reserved_slots_str = reserved_slots::get(matches);
        let reserved_slots = reserved_slots::parse_reserved_slots(reserved_slots_str)?;

        let params = TransactionBuilderParams::AddBid {
            public_key,
            delegation_rate,
            amount,
            minimum_delegation_amount,
            maximum_delegation_amount,
            reserved_slots,
        };

        let transaction_str_params = build_transaction_str_params(matches, ACCEPT_SESSION_ARGS);

        Ok((params, transaction_str_params))
    }

    fn add_args(add_bid_subcommand: Command) -> Command {
        add_bid_subcommand
            .arg(delegation_rate::arg())
            .arg(public_key::arg(DisplayOrder::PublicKey as usize))
            .arg(transaction_amount::arg())
            .arg(minimum_delegation_amount::arg())
            .arg(maximum_delegation_amount::arg())
            .arg(reserved_slots::arg())
    }
}

pub(super) mod activate_bid {
    use super::*;
    use casper_client::cli::{CliError, TransactionBuilderParams, TransactionStrParams};

    pub const NAME: &str = "activate-bid";

    const ACCEPT_SESSION_ARGS: bool = false;

    const ABOUT: &str = "Creates a new activate-bid transaction";
    pub fn build() -> Command {
        apply_common_creation_options(
            add_args(Command::new(NAME).about(ABOUT)),
            false,
            false,
            ACCEPT_SESSION_ARGS,
        )
    }

    pub fn put_transaction_build() -> Command {
        add_rpc_args(build())
    }

    pub fn run(
        matches: &ArgMatches,
    ) -> Result<(TransactionBuilderParams, TransactionStrParams), CliError> {
        let validator_str = validator::get(matches);
        let validator = public_key::parse_public_key(validator_str)?;

        let params = TransactionBuilderParams::ActivateBid { validator };

        let transaction_str_params = build_transaction_str_params(matches, ACCEPT_SESSION_ARGS);

        Ok((params, transaction_str_params))
    }

    fn add_args(activate_bid_subcommand: Command) -> Command {
        activate_bid_subcommand.arg(validator::arg())
    }
}

pub(super) mod withdraw_bid_all {
    use super::*;
    use crate::cli::TransactionBuilderParams;
    use casper_client::cli::CliError;
    use casper_types::U512;

    pub const NAME: &str = "withdraw-bid-all";

    const ACCEPT_SESSION_ARGS: bool = false;

    const ABOUT: &str =
        "Creates a new withdraw-bid transaction which completely unbonds the validator";
    pub fn build() -> Command {
        apply_common_creation_options(
            add_args(Command::new(NAME).about(ABOUT)),
            false,
            false,
            ACCEPT_SESSION_ARGS,
        )
    }

    pub fn put_transaction_build() -> Command {
        add_rpc_args(build())
    }

    pub fn run(
        matches: &ArgMatches,
        amount: U512,
    ) -> Result<(TransactionBuilderParams, TransactionStrParams), CliError> {
        let public_key_str = public_key::get(matches)?;
        let public_key = public_key::parse_public_key(&public_key_str)?;

        let params = TransactionBuilderParams::WithdrawBid {
            public_key,
            amount,
            min_bid_override: true,
        };
        let transaction_str_params = build_transaction_str_params(matches, ACCEPT_SESSION_ARGS);

        Ok((params, transaction_str_params))
    }

    fn add_args(withdraw_bid_subcommand: Command) -> Command {
        withdraw_bid_subcommand.arg(public_key::arg(DisplayOrder::PublicKey as usize))
    }
}

pub(super) mod withdraw_bid {
    use super::*;
    use crate::cli::TransactionBuilderParams;
    use casper_client::cli::CliError;

    pub const NAME: &str = "withdraw-bid";

    const ACCEPT_SESSION_ARGS: bool = false;

    const ABOUT: &str = "Creates a new withdraw-bid transaction";
    pub fn build() -> Command {
        apply_common_creation_options(
            add_args(Command::new(NAME).about(ABOUT)),
            false,
            false,
            ACCEPT_SESSION_ARGS,
        )
    }

    pub fn put_transaction_build() -> Command {
        add_rpc_args(build())
    }

    pub fn run(
        matches: &ArgMatches,
    ) -> Result<(TransactionBuilderParams, TransactionStrParams), CliError> {
        let public_key_str = public_key::get(matches)?;
        let public_key = public_key::parse_public_key(&public_key_str)?;

        let amount_str = transaction_amount::get(matches);
        let amount = transaction_amount::parse_transaction_amount(amount_str)?;

        let min_bid_override = min_bid_override::get(matches);

        let params = TransactionBuilderParams::WithdrawBid {
            public_key,
            amount,
            min_bid_override,
        };
        let transaction_str_params = build_transaction_str_params(matches, ACCEPT_SESSION_ARGS);

        Ok((params, transaction_str_params))
    }

    fn add_args(withdraw_bid_subcommand: Command) -> Command {
        withdraw_bid_subcommand
            .arg(public_key::arg(DisplayOrder::PublicKey as usize))
            .arg(min_bid_override::arg())
            .arg(transaction_amount::arg())
    }
}

pub(super) mod delegate {
    use super::*;
    use casper_client::cli::{CliError, TransactionBuilderParams};

    pub const NAME: &str = "delegate";

    const ACCEPT_SESSION_ARGS: bool = false;

    const ABOUT: &str = "Creates a new delegate transaction";

    pub fn build() -> Command {
        apply_common_creation_options(
            add_args(Command::new(NAME).about(ABOUT)),
            false,
            false,
            ACCEPT_SESSION_ARGS,
        )
    }

    pub fn put_transaction_build() -> Command {
        add_rpc_args(build())
    }

    pub fn run(
        matches: &ArgMatches,
    ) -> Result<(TransactionBuilderParams, TransactionStrParams), CliError> {
        let delegator_str = delegator::get(matches);
        let delegator = public_key::parse_public_key(delegator_str)?;

        let validator_str = validator::get(matches);
        let validator = public_key::parse_public_key(validator_str)?;

        let amount_str = transaction_amount::get(matches);
        let amount = transaction_amount::parse_transaction_amount(amount_str)?;

        let params = TransactionBuilderParams::Delegate {
            delegator,
            validator,
            amount,
        };
        let transaction_str_params = build_transaction_str_params(matches, ACCEPT_SESSION_ARGS);

        Ok((params, transaction_str_params))
    }

    fn add_args(delegate_subcommand: Command) -> Command {
        delegate_subcommand
            .arg(delegator::arg())
            .arg(validator::arg())
            .arg(transaction_amount::arg())
    }
}

pub(super) mod undelegate {
    use super::*;
    use casper_client::cli::{CliError, TransactionBuilderParams};

    pub const NAME: &str = "undelegate";

    const ACCEPT_SESSION_ARGS: bool = false;

    const ABOUT: &str = "Creates a new undelegate transaction";

    pub fn build() -> Command {
        apply_common_creation_options(
            add_args(Command::new(NAME).about(ABOUT)),
            false,
            false,
            ACCEPT_SESSION_ARGS,
        )
    }

    pub fn put_transaction_build() -> Command {
        add_rpc_args(build())
    }

    pub fn run(
        matches: &ArgMatches,
    ) -> Result<(TransactionBuilderParams, TransactionStrParams), CliError> {
        let delegator_str = delegator::get(matches);
        let delegator = public_key::parse_public_key(delegator_str)?;

        let validator_str = validator::get(matches);
        let validator = public_key::parse_public_key(validator_str)?;

        let amount_str = transaction_amount::get(matches);
        let amount = transaction_amount::parse_transaction_amount(amount_str)?;

        let params = TransactionBuilderParams::Undelegate {
            delegator,
            validator,
            amount,
        };
        let transaction_str_params = build_transaction_str_params(matches, ACCEPT_SESSION_ARGS);
        Ok((params, transaction_str_params))
    }

    fn add_args(undelegate_subcommand: Command) -> Command {
        undelegate_subcommand
            .arg(delegator::arg())
            .arg(validator::arg())
            .arg(transaction_amount::arg())
    }
}

pub(super) mod redelegate {
    use super::*;
    use casper_client::cli::{CliError, TransactionBuilderParams};

    pub const NAME: &str = "redelegate";

    const ACCEPT_SESSION_ARGS: bool = false;
    const ABOUT: &str = "Creates a new redelegate transaction";

    pub fn build() -> Command {
        apply_common_creation_options(
            add_args(Command::new(NAME).about(ABOUT)),
            false,
            false,
            ACCEPT_SESSION_ARGS,
        )
    }

    pub fn put_transaction_build() -> Command {
        add_rpc_args(build())
    }

    pub fn run(
        matches: &ArgMatches,
    ) -> Result<(TransactionBuilderParams, TransactionStrParams), CliError> {
        let delegator_str = delegator::get(matches);
        let delegator = public_key::parse_public_key(delegator_str)?;

        let validator_str = validator::get(matches);
        let validator = public_key::parse_public_key(validator_str)?;

        let new_validator_str = new_validator::get(matches);
        let new_validator = public_key::parse_public_key(new_validator_str)?;

        let amount_str = transaction_amount::get(matches);
        let amount = transaction_amount::parse_transaction_amount(amount_str)?;

        let params = TransactionBuilderParams::Redelegate {
            delegator,
            validator,
            new_validator,
            amount,
        };
        let transaction_str_params = build_transaction_str_params(matches, ACCEPT_SESSION_ARGS);
        Ok((params, transaction_str_params))
    }

    fn add_args(redelegate_subcommand: Command) -> Command {
        redelegate_subcommand
            .arg(delegator::arg())
            .arg(validator::arg())
            .arg(new_validator::arg())
            .arg(transaction_amount::arg())
    }
}

pub(super) mod change_bid_public_key {
    use super::*;
    use casper_client::cli::{CliError, TransactionBuilderParams, TransactionStrParams};

    pub const NAME: &str = "change-bid-public-key";

    const ACCEPT_SESSION_ARGS: bool = false;

    const ABOUT: &str = "Creates a new change-bid-public-key transaction";
    pub fn build() -> Command {
        apply_common_creation_options(
            add_args(Command::new(NAME).about(ABOUT)),
            false,
            false,
            ACCEPT_SESSION_ARGS,
        )
    }

    pub fn put_transaction_build() -> Command {
        add_rpc_args(build())
    }

    pub fn run(
        matches: &ArgMatches,
    ) -> Result<(TransactionBuilderParams, TransactionStrParams), CliError> {
        let public_key_str = public_key::get(matches)?;
        let public_key = public_key::parse_public_key(&public_key_str)?;

        let new_public_key_str = new_public_key::get(matches)?;
        let new_public_key = new_public_key::parse_public_key(&new_public_key_str)?;

        let params = TransactionBuilderParams::ChangeBidPublicKey {
            public_key,
            new_public_key,
        };

        let transaction_str_params = build_transaction_str_params(matches, ACCEPT_SESSION_ARGS);

        Ok((params, transaction_str_params))
    }

    fn add_args(change_bid_public_key_subcommand: Command) -> Command {
        change_bid_public_key_subcommand
            .arg(public_key::arg(DisplayOrder::PublicKey as usize))
            .arg(new_public_key::arg(DisplayOrder::NewPublicKey as usize))
    }
}

pub(super) mod add_reservations {
    use super::*;
    use casper_client::cli::{CliError, TransactionBuilderParams, TransactionStrParams};

    pub const NAME: &str = "add-reservations";

    const ACCEPT_SESSION_ARGS: bool = false;

    const ABOUT: &str = "Creates a new add-reservations transaction";
    pub fn build() -> Command {
        apply_common_creation_options(
            add_args(Command::new(NAME).about(ABOUT)),
            false,
            false,
            ACCEPT_SESSION_ARGS,
        )
    }

    pub fn put_transaction_build() -> Command {
        add_rpc_args(build())
    }

    pub fn run(
        matches: &ArgMatches,
    ) -> Result<(TransactionBuilderParams, TransactionStrParams), CliError> {
        let reservations_str = reservations::get(matches);
        let reservations = reservations::parse_reservations(reservations_str)?;

        let params = TransactionBuilderParams::AddReservations { reservations };

        let transaction_str_params = build_transaction_str_params(matches, ACCEPT_SESSION_ARGS);

        Ok((params, transaction_str_params))
    }

    fn add_args(add_reservations_subcommand: Command) -> Command {
        add_reservations_subcommand.arg(reservations::arg())
    }
}

pub(super) mod cancel_reservations {
    use super::*;
    use casper_client::cli::{CliError, TransactionBuilderParams, TransactionStrParams};

    pub const NAME: &str = "cancel-reservations";

    const ACCEPT_SESSION_ARGS: bool = false;

    const ABOUT: &str = "Creates a new cancel-reservations transaction";
    pub fn build() -> Command {
        apply_common_creation_options(
            add_args(Command::new(NAME).about(ABOUT)),
            false,
            false,
            ACCEPT_SESSION_ARGS,
        )
    }

    pub fn put_transaction_build() -> Command {
        add_rpc_args(build())
    }

    pub fn run(
        matches: &ArgMatches,
    ) -> Result<(TransactionBuilderParams, TransactionStrParams), CliError> {
        let validator_str = validator::get(matches);
        let validator = public_key::parse_public_key(validator_str)?;

        let delegators_str = delegators::get(matches);
        let delegators = delegators::parse_delegators(delegators_str)?;

        let params = TransactionBuilderParams::CancelReservations {
            validator,
            delegators,
        };

        let transaction_str_params = build_transaction_str_params(matches, ACCEPT_SESSION_ARGS);

        Ok((params, transaction_str_params))
    }

    fn add_args(cancel_reservations_subcommand: Command) -> Command {
        cancel_reservations_subcommand
            .arg(validator::arg())
            .arg(delegators::arg())
    }
}

pub(super) mod invocable_entity {
    use super::*;
    use casper_client::cli::{CliError, TransactionBuilderParams};

    pub const NAME: &str = "invocable-entity";
    const ACCEPT_SESSION_ARGS: bool = true;

    const ABOUT: &str = "Creates a new transaction targeting an invocable entity";

    pub fn build() -> Command {
        apply_common_creation_options(
            add_args(Command::new(NAME).about(ABOUT)),
            false,
            false,
            ACCEPT_SESSION_ARGS,
        )
    }

    pub fn put_transaction_build() -> Command {
        add_rpc_args(build())
    }

    pub fn run(
        matches: &ArgMatches,
    ) -> Result<(TransactionBuilderParams, TransactionStrParams), CliError> {
        show_simple_arg_examples_and_exit_if_required(matches);
        show_json_args_examples_and_exit_if_required(matches);

        let entity_addr = match contract_hash::get(matches) {
            None => {
                let entity_addr_str = entity_addr::get(matches)?;
                entity_addr::parse_entity_addr(entity_addr_str)?
            }
            Some(contract_hash_as_str) => contract_hash::parse_contract_hash(contract_hash_as_str)?,
        };

        let entry_point = session_entry_point::get(matches).unwrap_or_default();
        let runtime = get_transaction_runtime(matches)?;
        let params = TransactionBuilderParams::InvocableEntity {
            entity_hash: entity_addr.into(), // TODO: Skip `entity_addr` and match directly for hash?
            entry_point,
            runtime,
        };
        let transaction_str_params = build_transaction_str_params(matches, ACCEPT_SESSION_ARGS);
        Ok((params, transaction_str_params))
    }

    fn add_args(invocable_entity_subcommand: Command) -> Command {
        invocable_entity_subcommand
            .arg(entity_addr::arg().required_unless_present(contract_hash::ARG_NAME))
            .arg(contract_hash::arg())
            .arg(session_entry_point::arg())
            .arg(transaction_runtime::arg())
            .arg(transferred_value::arg())
            .arg(chunked_args::arg())
    }
}

pub(super) mod invocable_entity_alias {
    use super::*;
    use casper_client::cli::{CliError, TransactionBuilderParams};

    pub const NAME: &str = "invocable-entity-alias";

    const ACCEPT_SESSION_ARGS: bool = true;

    const ABOUT: &str = "Creates a new transaction targeting an invocable entity via its alias";

    pub fn build() -> Command {
        apply_common_creation_options(
            add_args(Command::new(NAME).about(ABOUT)),
            false,
            false,
            ACCEPT_SESSION_ARGS,
        )
    }

    pub fn put_transaction_build() -> Command {
        add_rpc_args(build())
    }

    pub fn run(
        matches: &ArgMatches,
    ) -> Result<(TransactionBuilderParams, TransactionStrParams), CliError> {
        show_simple_arg_examples_and_exit_if_required(matches);
        show_json_args_examples_and_exit_if_required(matches);

        let entity_alias = entity_alias_arg::get(matches);
        let entry_point = session_entry_point::get(matches).unwrap_or_default();
        let runtime = get_transaction_runtime(matches)?;
        let params = TransactionBuilderParams::InvocableEntityAlias {
            entity_alias,
            entry_point,
            runtime,
        };
        let transaction_str_params = build_transaction_str_params(matches, ACCEPT_SESSION_ARGS);
        Ok((params, transaction_str_params))
    }

    fn add_args(invocable_entity_alias_subcommand: Command) -> Command {
        invocable_entity_alias_subcommand
            .arg(entity_alias_arg::arg())
            .arg(session_entry_point::arg())
    }
}

pub(super) mod package {
    use super::*;
    use casper_client::cli::{CliError, TransactionBuilderParams};

    pub const NAME: &str = "package";

    const ACCEPT_SESSION_ARGS: bool = true;

    const ABOUT: &str = "Creates a new transaction targeting a package";

    pub fn build() -> Command {
        apply_common_creation_options(
            add_args(Command::new(NAME).about(ABOUT)),
            false,
            false,
            ACCEPT_SESSION_ARGS,
        )
    }

    pub fn put_transaction_build() -> Command {
        add_rpc_args(build())
    }

    pub fn run(
        matches: &ArgMatches,
    ) -> Result<(TransactionBuilderParams, TransactionStrParams), CliError> {
        show_simple_arg_examples_and_exit_if_required(matches);
        show_json_args_examples_and_exit_if_required(matches);

        let package_addr = match contract_package_hash::get(matches) {
            None => {
                let maybe_package_addr_str = package_addr::get(matches);
                package_addr::parse_package_addr(maybe_package_addr_str)?
            }
            Some(contract_package_as_str) => {
                contract_package_hash::parse_package_addr(contract_package_as_str)?
            }
        };
        let maybe_entity_version = session_version::get(matches);
        let major_protocol_version = major_version::get(matches);
        let runtime = get_transaction_runtime(matches)?;

        let entry_point = session_entry_point::get(matches).unwrap_or_default();
        let params = TransactionBuilderParams::PackageWithMajorVersion {
            package_hash: package_addr.into(), // TODO: Skip `package_addr` and match directly for hash?
            maybe_entity_version,
            entry_point,
            runtime,
            major_protocol_version,
        };
        let transaction_str_params = build_transaction_str_params(matches, ACCEPT_SESSION_ARGS);
        Ok((params, transaction_str_params))
    }

    fn add_args(package_subcommand: Command) -> Command {
        package_subcommand
            .arg(package_addr::arg().required_unless_present(contract_package_hash::ARG_NAME))
            .arg(contract_package_hash::arg())
            .arg(session_version::arg())
            .arg(transaction_runtime::arg())
            .arg(transferred_value::arg())
            .arg(chunked_args::arg())
            .arg(major_version::arg())
            .arg(session_entry_point::arg())
    }
}

pub(super) mod package_alias {
    use super::*;
    use casper_client::cli::{CliError, TransactionBuilderParams};

    pub const NAME: &str = "package-name";

    const ACCEPT_SESSION_ARGS: bool = true;

    const ABOUT: &str = "Creates a new transaction targeting package via its alias";

    pub fn build() -> Command {
        apply_common_creation_options(
            add_args(Command::new(NAME).about(ABOUT)),
            false,
            false,
            ACCEPT_SESSION_ARGS,
        )
    }

    pub fn put_transaction_build() -> Command {
        add_rpc_args(build())
    }

    pub fn run(
        matches: &ArgMatches,
    ) -> Result<(TransactionBuilderParams, TransactionStrParams), CliError> {
        show_simple_arg_examples_and_exit_if_required(matches);
        show_json_args_examples_and_exit_if_required(matches);

        let package_alias = package_name_arg::get(matches);

        let maybe_entity_version = session_version::get(matches);
        let major_protocol_version = major_version::get(matches);

        let entry_point = session_entry_point::get(matches).unwrap_or_default();
        let runtime = get_transaction_runtime(matches)?;

        let params = TransactionBuilderParams::PackageAliasWithMajorVersion {
            package_alias,
            maybe_entity_version,
            entry_point,
            runtime,
            major_protocol_version,
        };
        let transaction_str_params = build_transaction_str_params(matches, ACCEPT_SESSION_ARGS);
        Ok((params, transaction_str_params))
    }

    fn add_args(package_alias_subcommand: Command) -> Command {
        package_alias_subcommand
            .arg(package_name_arg::arg())
            .arg(session_version::arg())
            .arg(transaction_runtime::arg())
            .arg(transferred_value::arg())
            .arg(chunked_args::arg())
            .arg(major_version::arg())
            .arg(session_entry_point::arg())
    }
}

pub(super) mod session {
    use super::*;
    use crate::cli::parse;
    use casper_client::cli::{CliError, TransactionBuilderParams};

    pub const NAME: &str = "session";

    const ACCEPT_SESSION_ARGS: bool = true;

    const ABOUT: &str = "Creates a new transaction for running session logic";

    pub fn build() -> Command {
        apply_common_creation_options(
            add_args(Command::new(NAME).about(ABOUT)),
            false,
            false,
            ACCEPT_SESSION_ARGS,
        )
    }

    pub fn put_transaction_build() -> Command {
        add_rpc_args(build())
    }

    pub fn run(
        matches: &ArgMatches,
    ) -> Result<(TransactionBuilderParams, TransactionStrParams), CliError> {
        show_simple_arg_examples_and_exit_if_required(matches);
        show_json_args_examples_and_exit_if_required(matches);

        let wasm_path_str = wasm_path::get(matches);

        if wasm_path_str.is_none() {
            return Err(CliError::InvalidArgument {
                context: "wasm_path",
                error: "Wasm path cannot be empty".to_string(),
            });
        }

        let transaction_bytes = parse::transaction_module_bytes(wasm_path_str.unwrap_or_default())?;

        let is_install_upgrade: bool = is_install_upgrade::get(matches);
        let runtime = get_transaction_runtime(matches)?;

        let params = TransactionBuilderParams::Session {
            is_install_upgrade,
            transaction_bytes,
            runtime,
        };
        let transaction_str_params = build_transaction_str_params(matches, ACCEPT_SESSION_ARGS);
        Ok((params, transaction_str_params))
    }

    fn add_args(session_subcommand: Command) -> Command {
        session_subcommand
            .arg(wasm_path::arg())
            .arg(session_entry_point::arg())
            .arg(is_install_upgrade::arg(
                DisplayOrder::IsInstallUpgrade as usize,
            ))
            .arg(transaction_runtime::arg())
            .arg(transferred_value::arg())
            .arg(chunked_args::arg())
    }
}

pub(super) mod transfer {
    use super::*;
    use crate::cli::parse;
    use casper_client::cli::{CliError, TransactionBuilderParams};
    use sha3::{Digest as _, Keccak256};

    pub const NAME: &str = "transfer";

    const ACCEPT_SESSION_ARGS: bool = false;
    const EVM_ADDRESS_LENGTH: usize = 20;
    const EVM_ADDRESS_HEX_LENGTH: usize = EVM_ADDRESS_LENGTH * 2;

    const ABOUT: &str = "Creates a new native transfer transaction";

    pub fn build() -> Command {
        apply_common_creation_options(
            add_args(Command::new(NAME).about(ABOUT)),
            false,
            false,
            ACCEPT_SESSION_ARGS,
        )
    }

    pub fn put_transaction_build() -> Command {
        add_rpc_args(build())
    }

    pub fn run(
        matches: &ArgMatches,
    ) -> Result<(TransactionBuilderParams, TransactionStrParams), CliError> {
        let source_str = source::get(matches);
        let maybe_source = if let Some(source) = source_str {
            Some(parse::uref(source)?)
        } else {
            None
        };

        let amount = transfer_amount::get(matches);
        let amount = transaction_amount::parse_transaction_amount(amount)?;

        let maybe_id = transfer_id::get(matches);

        let target_str = target::get(matches);
        let params = match parse::transfer_target(target_str) {
            Ok(target) => TransactionBuilderParams::Transfer {
                maybe_source,
                target,
                amount,
                maybe_id,
            },
            Err(native_error) => match parse_evm_address(target_str)? {
                Some(target) => TransactionBuilderParams::EvmTransfer {
                    maybe_source,
                    target,
                    amount,
                    maybe_id,
                },
                None => return Err(native_error),
            },
        };
        let transaction_str_params = build_transaction_str_params(matches, ACCEPT_SESSION_ARGS);

        Ok((params, transaction_str_params))
    }

    pub(super) fn parse_evm_address(
        value: &str,
    ) -> Result<Option<[u8; EVM_ADDRESS_LENGTH]>, CliError> {
        let trimmed = value.trim();
        let Some(hex) = trimmed
            .strip_prefix("0x")
            .or_else(|| trimmed.strip_prefix("0X"))
        else {
            if trimmed.len() == EVM_ADDRESS_HEX_LENGTH
                && trimmed.bytes().all(|byte| byte.is_ascii_hexdigit())
            {
                return Err(invalid_evm_address(
                    "EVM address targets must include a 0x prefix",
                ));
            }
            return Ok(None);
        };

        if hex.len() != EVM_ADDRESS_HEX_LENGTH {
            return Err(invalid_evm_address(
                "EVM address targets must contain exactly 40 hexadecimal characters after the 0x prefix",
            ));
        }
        if !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(invalid_evm_address(
                "invalid hexadecimal EVM address target",
            ));
        }
        if has_mixed_hex_case(hex) && !has_valid_eip55_checksum(hex) {
            return Err(invalid_evm_address(
                "invalid EIP-55 checksum for EVM address target",
            ));
        }

        let bytes = base16::decode(hex)
            .map_err(|_| invalid_evm_address("invalid hexadecimal EVM address target"))?;
        let mut address = [0u8; EVM_ADDRESS_LENGTH];
        address.copy_from_slice(&bytes);
        Ok(Some(address))
    }

    fn invalid_evm_address(error: &str) -> CliError {
        CliError::InvalidArgument {
            context: "target",
            error: error.to_string(),
        }
    }

    fn has_mixed_hex_case(value: &str) -> bool {
        let has_lower = value.bytes().any(|byte| byte.is_ascii_lowercase());
        let has_upper = value.bytes().any(|byte| byte.is_ascii_uppercase());
        has_lower && has_upper
    }

    fn has_valid_eip55_checksum(value: &str) -> bool {
        let lower = value.to_ascii_lowercase();
        let hash = Keccak256::digest(lower.as_bytes());
        value.bytes().enumerate().all(|(index, byte)| {
            if !byte.is_ascii_alphabetic() {
                return true;
            }
            let hash_byte = hash[index / 2];
            let hash_nibble = if index % 2 == 0 {
                hash_byte >> 4
            } else {
                hash_byte & 0x0f
            };
            byte.is_ascii_uppercase() == (hash_nibble >= 8)
        })
    }

    fn add_args(transfer_subcommand: Command) -> Command {
        transfer_subcommand
            .arg(source::arg())
            .arg(target::arg())
            .arg(transfer_amount::arg())
            .arg(transfer_id::arg())
    }
}

pub(super) mod source {
    use crate::transaction::creation_common::DisplayOrder;
    use clap::{Arg, ArgMatches};

    pub const ARG_NAME: &str = "source";
    const ARG_VALUE_NAME: &str = "FORMATTED STRING";
    const ARG_HELP: &str = "the hex string representing the source URef for the transfer";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::Source as usize)
    }

    pub fn get(matches: &ArgMatches) -> Option<&str> {
        matches.get_one::<String>(ARG_NAME).map(String::as_str)
    }
}

pub(super) mod target {
    use crate::transaction::creation_common::DisplayOrder;
    use clap::{Arg, ArgMatches};

    pub const ARG_NAME: &str = "target";
    const ARG_VALUE_NAME: &str = "FORMATTED STRING";
    const ARG_HELP: &str =
        "the public key, account hash, URef, or 0x-prefixed 20-byte EVM address \
        representing the transfer target";

    pub fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .required(true)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::Target as usize)
    }

    pub fn get(matches: &ArgMatches) -> &str {
        matches
            .get_one::<String>(ARG_NAME)
            .map(String::as_str)
            .unwrap_or_else(|| panic!("should have {} arg", ARG_NAME))
    }
}

/// Handles providing the arg for speculative execution.
pub(super) mod speculative_exec {
    use super::*;

    const ARG_NAME: &str = "speculative-exec";
    const ARG_HELP: &str =
        "If the receiving node supports this, execution of the deploy will only be attempted on \
        that single node. Full validation of the deploy is not performed, and successful execution \
        at the given global state is no guarantee that the deploy will be able to be successfully \
        executed if put to the network, nor should execution costs be expected to be identical.";

    pub(in crate::transaction) fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .required(false)
            .num_args(0)
            .help(ARG_HELP)
            .display_order(DisplayOrder::SpeculativeExec as usize)
    }

    // get: The command line posibilities are encoded by a boolean
    // false represents no --speculative-exec argument at all
    // true represents a --speculative-exec with argument
    pub(in crate::transaction) fn get(matches: &ArgMatches) -> bool {
        matches.get_flag(ARG_NAME)
    }
}

/// Handles providing the arg for and retrieval of the transfer id.
pub(super) mod transfer_id {
    use super::*;

    pub(in crate::transaction) const ARG_NAME: &str = "transfer-id";
    const ARG_SHORT: char = 'i';
    const ARG_VALUE_NAME: &str = "64-BIT INTEGER";
    const ARG_HELP: &str = "User-defined identifier, permanently associated with the transfer";

    pub(in crate::transaction) fn arg() -> Arg {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .short(ARG_SHORT)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::TransferId as usize)
    }

    pub(in crate::transaction) fn get(matches: &ArgMatches) -> Option<u64> {
        matches.get_one::<u64>(ARG_NAME).map(get_deref_helper)
    }
    fn get_deref_helper(get_result: &u64) -> u64 {
        *get_result
    }
}

pub(super) fn build_transaction_str_params(
    matches: &ArgMatches,
    obtain_session_args: bool,
) -> TransactionStrParams {
    let secret_key = common::secret_key::get(matches).unwrap_or_default();
    let timestamp = timestamp::get(matches);
    let ttl = ttl::get(matches);
    let chain_name = chain_name::get(matches);
    let maybe_pricing_mode = pricing_mode::get(matches);
    let gas_price_tolerance = gas_price_tolerance::get(matches);
    let additional_computation_factor = additional_computation_factor::get(matches);
    let payment_amount = payment_amount::get(matches);
    let receipt = receipt::get(matches);
    let standard_payment = standard_payment::get(matches);

    let maybe_output_path = output::get(matches).unwrap_or_default();
    let initiator_addr = initiator_address::get(matches);

    if obtain_session_args {
        let session_args_simple = arg_simple::session::get(matches);
        let session_args_json = args_json::session::get(matches);
        let session_entry_point = session_entry_point::get(matches);
        let chunked_args = chunked_args::get(matches);
        TransactionStrParams {
            secret_key,
            timestamp,
            ttl,
            chain_name,
            initiator_addr,
            session_args_simple,
            session_args_json,
            pricing_mode: maybe_pricing_mode.map(|pm| pm.as_str()).unwrap_or_default(),
            output_path: maybe_output_path,
            payment_amount,
            gas_price_tolerance,
            additional_computation_factor,
            receipt,
            standard_payment,
            transferred_value: transferred_value::get_raw(matches)
                .map(|tv| tv.as_str())
                .unwrap_or_default(),
            session_entry_point,
            chunked_args,
        }
    } else {
        TransactionStrParams {
            secret_key,
            timestamp,
            ttl,
            chain_name,
            initiator_addr,
            pricing_mode: maybe_pricing_mode.map(|pm| pm.as_str()).unwrap_or_default(),
            output_path: maybe_output_path,
            payment_amount,
            gas_price_tolerance,
            additional_computation_factor,
            receipt,
            standard_payment,
            ..Default::default()
        }
    }
}
pub(super) fn add_rpc_args(subcommand: Command) -> Command {
    subcommand
        .arg(common::rpc_id::arg(DisplayOrder::RpcId as usize))
        .arg(common::node_address::arg(
            DisplayOrder::NodeAddress as usize,
        ))
        .arg(common::verbose::arg(DisplayOrder::Verbose as usize))
}

fn get_transaction_runtime(matches: &ArgMatches) -> Result<TransactionRuntimeParams, CliError> {
    let runtime_tag = transaction_runtime::get(matches)
        .cloned()
        .unwrap_or_default();
    let maybe_transferred_value = transferred_value::get(matches)?;
    let runtime = match runtime_tag {
        TransactionRuntime::VmCasperV1 => {
            if maybe_transferred_value.is_some() {
                Err(CliError::InvalidArgument {
                    context: "transferred_value",
                    error: "Argument `transferred-value` has no usage in V1 execution engine VM (if you provided no execution engine vm parameter, V1 is the default)".to_string(),
                })?;
            }
            TransactionRuntimeParams::VmCasperV1
        }
        TransactionRuntime::VmCasperV2 => {
            if maybe_transferred_value.is_none() {
                Err(CliError::InvalidArgument {
                    context: "transferred_value",
                    error: "VmCasperV2 requires `transferred-value` argument".to_string(),
                })?;
            }
            TransactionRuntimeParams::VmCasperV2 {
                transferred_value: maybe_transferred_value.unwrap(),
                seed: None,
            }
        }
    };
    Ok(runtime)
}

#[cfg(test)]
mod tests {
    use super::{is_install_upgrade, transfer};
    use casper_client::cli::TransactionBuilderParams;
    use casper_types::TransferTarget;
    use clap::{ArgMatches, Command};

    // Helper function to build a command with `is_install_upgrade` argument
    fn build_app() -> Command {
        Command::new("put-transaction session").arg(is_install_upgrade::arg(1))
    }

    fn transfer_matches(command: Command, target: &str) -> ArgMatches {
        command
            .try_get_matches_from(vec![
                "transfer",
                "--target",
                target,
                "--transfer-amount",
                "500000000",
                "--chain-name",
                "casper-net-1",
                "--gas-price-tolerance",
                "1",
                "--initiator-address",
                "01722e1b3d31bef0ba832121bd2941aae6a246d0d05ac95aa16dd587cc5469871d",
            ])
            .unwrap()
    }

    #[test]
    fn test_is_install_upgrade_flag_present() {
        // Simulate running with the `--install-upgrade` flag
        let matches = build_app()
            .try_get_matches_from(vec!["put-transaction session", "--install-upgrade"])
            .unwrap();

        // Assert that `get` returns true when the flag is present
        assert!(is_install_upgrade::get(&matches));
    }

    #[test]
    fn test_is_install_upgrade_flag_absent() {
        // Simulate running without the `--install-upgrade` flag
        let matches = build_app()
            .try_get_matches_from(vec!["put-transaction session"])
            .unwrap();

        // Assert that `get` returns false when the flag is absent
        assert!(!is_install_upgrade::get(&matches));
    }

    #[test]
    fn parses_lowercase_evm_address() {
        let parsed = transfer::parse_evm_address("0xde709f2102306220921060314715629080e2fb77")
            .unwrap()
            .unwrap();
        assert_eq!(
            parsed,
            [
                0xde, 0x70, 0x9f, 0x21, 0x02, 0x30, 0x62, 0x20, 0x92, 0x10, 0x60, 0x31, 0x47, 0x15,
                0x62, 0x90, 0x80, 0xe2, 0xfb, 0x77,
            ]
        );
    }

    #[test]
    fn parses_uppercase_evm_address_and_prefix() {
        let parsed = transfer::parse_evm_address("0XDE709F2102306220921060314715629080E2FB77")
            .unwrap()
            .unwrap();
        assert_eq!(
            parsed,
            [
                0xde, 0x70, 0x9f, 0x21, 0x02, 0x30, 0x62, 0x20, 0x92, 0x10, 0x60, 0x31, 0x47, 0x15,
                0x62, 0x90, 0x80, 0xe2, 0xfb, 0x77,
            ]
        );
    }

    #[test]
    fn parses_eip55_checksummed_evm_address() {
        let parsed = transfer::parse_evm_address("0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed")
            .unwrap()
            .unwrap();
        assert_eq!(
            parsed,
            [
                0x5a, 0xae, 0xb6, 0x05, 0x3f, 0x3e, 0x94, 0xc9, 0xb9, 0xa0, 0x9f, 0x33, 0x66, 0x94,
                0x35, 0xe7, 0xef, 0x1b, 0xea, 0xed,
            ]
        );
    }

    #[test]
    fn rejects_evm_address_without_prefix() {
        let error =
            transfer::parse_evm_address("de709f2102306220921060314715629080e2fb77").unwrap_err();
        assert!(error.to_string().contains("must include a 0x prefix"));
    }

    #[test]
    fn rejects_invalid_eip55_checksum() {
        let error =
            transfer::parse_evm_address("0x52908400098527886E0F7030069857D2E4169Ee7").unwrap_err();
        assert!(error.to_string().contains("invalid EIP-55 checksum"));
    }

    #[test]
    fn rejects_malformed_evm_address_hex() {
        let error =
            transfer::parse_evm_address("0xde709f2102306220921060314715629080e2fb7z").unwrap_err();
        assert!(error.to_string().contains("invalid hexadecimal"));
    }

    #[test]
    fn rejects_incorrect_evm_address_lengths() {
        let short = transfer::parse_evm_address("0x1234").unwrap_err();
        assert!(short.to_string().contains("exactly 40"));

        let long = transfer::parse_evm_address(&format!("0x{}", "11".repeat(21))).unwrap_err();
        assert!(long.to_string().contains("exactly 40"));
    }

    #[test]
    fn make_and_put_transfer_commands_accept_evm_target() {
        let target = "0xde709f2102306220921060314715629080e2fb77";
        for command in [transfer::build(), transfer::put_transaction_build()] {
            let matches = transfer_matches(command, target);
            let (params, _) = transfer::run(&matches).unwrap();
            assert!(matches!(
                params,
                TransactionBuilderParams::EvmTransfer {
                    target: [
                        0xde, 0x70, 0x9f, 0x21, 0x02, 0x30, 0x62, 0x20, 0x92, 0x10, 0x60, 0x31,
                        0x47, 0x15, 0x62, 0x90, 0x80, 0xe2, 0xfb, 0x77,
                    ],
                    ..
                }
            ));
        }
    }

    #[test]
    fn native_transfer_target_uses_existing_params() {
        let target = "uref-0202030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20-007";
        let matches = transfer_matches(transfer::build(), target);
        let (params, _) = transfer::run(&matches).unwrap();
        assert!(matches!(
            params,
            TransactionBuilderParams::Transfer {
                target: TransferTarget::URef(_),
                ..
            }
        ));
    }
}
