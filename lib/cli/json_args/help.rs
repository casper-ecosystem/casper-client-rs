//! Functions for use in help commands.

use std::fmt::Write;

use once_cell::sync::Lazy;

// The default terminal width used by Clap.
const MAX_INFO_LINE_LENGTH: usize = 100;

struct InfoAndExamples {
    info: &'static str,
    examples: Vec<&'static str>,
}

static ALL_INFO_AND_EXAMPLES: Lazy<Vec<InfoAndExamples>> = Lazy::new(|| {
    vec![
        InfoAndExamples {
            info: "CLType Bool is represented as a JSON Bool, e.g.",
            examples: vec![r#"{"name":"entry_point_name","type":"Bool","value":false}"#],
        },
        InfoAndExamples {
            info: "CLTypes I32, I64, U8, U32 and U64 are represented as a JSON Number, e.g.",
            examples: vec![
                r#"{"name":"entry_point_name","type":"I32","value":-1}"#,
                r#"{"name":"entry_point_name","type":"I64","value":-2}"#,
                r#"{"name":"entry_point_name","type":"U8","value":1}"#,
                r#"{"name":"entry_point_name","type":"U32","value":2}"#,
                r#"{"name":"entry_point_name","type":"U64","value":3}"#,
            ],
        },
        InfoAndExamples {
            info: "CLTypes U128, U256 and U512 are represented as a JSON String of the decimal \
                value, or can be represented as a Number if the value is not more than u64::MAX \
                (18446744073709551615), e.g.",
            examples: vec![
                r#"{"name":"entry_point_name","type":"U128","value":1}"#,
                r#"{"name":"entry_point_name","type":"U128","value":"20000000000000000000"}"#,
                r#"{"name":"entry_point_name","type":"U256","value":2}"#,
                r#"{"name":"entry_point_name","type":"U256","value":"20000000000000000000"}"#,
                r#"{"name":"entry_point_name","type":"U512","value":3}"#,
                r#"{"name":"entry_point_name","type":"U512","value":"20000000000000000000"}"#,
            ],
        },
        InfoAndExamples {
            info: "CLType Unit is represented as a JSON null, e.g.",
            examples: vec![r#"{"name":"entry_point_name","type":"Unit","value":null}"#],
        },
        InfoAndExamples {
            info: "CLType String is represented as a JSON String, e.g.",
            examples: vec![r#"{"name":"entry_point_name","type":"String","value":"a"}"#],
        },
        InfoAndExamples {
            info: "CLType Key is represented as a JSON String (where the value is a properly \
                formatted string representation of a Key) or may also be represented as a JSON \
                Object of the form {\"<KEY VARIANT>\":\"<KEY AS FORMATTED STRING>\"}, e.g.",
            examples: vec![
                r#"{"name":"entry_point_name","type":"Key","value":"account-hash-201f1e1d1c1b1a191817161514131211100f0e0d0c0b0a090807060504030201"}"#,
                r#"{"name":"entry_point_name","type":"Key","value":{"Account":"account-hash-201f1e1d1c1b1a191817161514131211100f0e0d0c0b0a090807060504030201"}}"#,
                r#"{"name":"entry_point_name","type":"Key","value":"hash-201f1e1d1c1b1a191817161514131211100f0e0d0c0b0a090807060504030201"}"#,
                r#"{"name":"entry_point_name","type":"Key","value":{"Hash":"hash-201f1e1d1c1b1a191817161514131211100f0e0d0c0b0a090807060504030201"}}"#,
                r#"{"name":"entry_point_name","type":"Key","value":"uref-201f1e1d1c1b1a191817161514131211100f0e0d0c0b0a090807060504030201-000"}"#,
                r#"{"name":"entry_point_name","type":"Key","value":{"URef":"uref-201f1e1d1c1b1a191817161514131211100f0e0d0c0b0a090807060504030201-000"}}"#,
                r#"{"name":"entry_point_name","type":"Key","value":"transfer-201f1e1d1c1b1a191817161514131211100f0e0d0c0b0a090807060504030201"}"#,
                r#"{"name":"entry_point_name","type":"Key","value":{"Transfer":"transfer-201f1e1d1c1b1a191817161514131211100f0e0d0c0b0a090807060504030201"}}"#,
                r#"{"name":"entry_point_name","type":"Key","value":"deploy-201f1e1d1c1b1a191817161514131211100f0e0d0c0b0a090807060504030201"}"#,
                r#"{"name":"entry_point_name","type":"Key","value":{"DeployInfo":"deploy-201f1e1d1c1b1a191817161514131211100f0e0d0c0b0a090807060504030201"}}"#,
                r#"{"name":"entry_point_name","type":"Key","value":"era-1"}"#,
                r#"{"name":"entry_point_name","type":"Key","value":{"EraInfo":"era-1"}}"#,
                r#"{"name":"entry_point_name","type":"Key","value":"balance-201f1e1d1c1b1a191817161514131211100f0e0d0c0b0a090807060504030201"}"#,
                r#"{"name":"entry_point_name","type":"Key","value":{"Balance":"balance-201f1e1d1c1b1a191817161514131211100f0e0d0c0b0a090807060504030201"}}"#,
                r#"{"name":"entry_point_name","type":"Key","value":"bid-201f1e1d1c1b1a191817161514131211100f0e0d0c0b0a090807060504030201"}"#,
                r#"{"name":"entry_point_name","type":"Key","value":{"Bid":"bid-201f1e1d1c1b1a191817161514131211100f0e0d0c0b0a090807060504030201"}}"#,
                r#"{"name":"entry_point_name","type":"Key","value":"withdraw-201f1e1d1c1b1a191817161514131211100f0e0d0c0b0a090807060504030201"}"#,
                r#"{"name":"entry_point_name","type":"Key","value":{"Withdraw":"withdraw-201f1e1d1c1b1a191817161514131211100f0e0d0c0b0a090807060504030201"}}"#,
                r#"{"name":"entry_point_name","type":"Key","value":"unbond-201f1e1d1c1b1a191817161514131211100f0e0d0c0b0a090807060504030201"}"#,
                r#"{"name":"entry_point_name","type":"Key","value":{"Unbond":"unbond-201f1e1d1c1b1a191817161514131211100f0e0d0c0b0a090807060504030201"}}"#,
                r#"{"name":"entry_point_name","type":"Key","value":"dictionary-201f1e1d1c1b1a191817161514131211100f0e0d0c0b0a090807060504030201"}"#,
                r#"{"name":"entry_point_name","type":"Key","value":{"Dictionary":"dictionary-201f1e1d1c1b1a191817161514131211100f0e0d0c0b0a090807060504030201"}}"#,
                r#"{"name":"entry_point_name","type":"Key","value":"system-entity-registry-0000000000000000000000000000000000000000000000000000000000000000"}"#,
                r#"{"name":"entry_point_name","type":"Key","value":{"SystemEntityRegistry":"system-entity-registry-0000000000000000000000000000000000000000000000000000000000000000"}}"#,
                r#"{"name":"entry_point_name","type":"Key","value":"chainspec-registry-1111111111111111111111111111111111111111111111111111111111111111"}"#,
                r#"{"name":"entry_point_name","type":"Key","value":{"ChainspecRegistry":"chainspec-registry-1111111111111111111111111111111111111111111111111111111111111111"}}"#,
            ],
        },
        InfoAndExamples {
            info:
                "CLTypes URef and PublicKey are represented as a JSON String where the value is a \
                properly formatted string representation of the respective type, e.g.",
            examples: vec![
                r#"{"name":"entry_point_name","type":"URef","value":"uref-201f1e1d1c1b1a191817161514131211100f0e0d0c0b0a090807060504030201-007"}"#,
                r#"{"name":"entry_point_name","type":"PublicKey","value":"017279ea868d185a40ed32ec076807c070de9c0fe986f5418c2aa71478f1e8ddf8"}"#,
                r#"{"name":"entry_point_name","type":"PublicKey","value":"02030963b980a774f9bf4fded595007b60045ca9593fe6d47296e4e1aaa2745c90d2"}"#,
            ],
        },
        InfoAndExamples {
            info: "CLType Option<T> is represented as a JSON null for None, or the JSON type \
                appropriate for the wrapped type T, e.g.",
            examples: vec![
                r#"{"name":"entry_point_name","type":{"Option":"U64"},"value":999}"#,
                r#"{"name":"entry_point_name","type":{"Option":"String"},"value":null}"#,
            ],
        },
        InfoAndExamples {
            info: "CLType List<T> is represented as a JSON Array where every element has a type \
                suitable to represent T.  For the special case of List<U8>, it can be represented \
                as a hex-encoded String, e.g.",
            examples: vec![
                r#"{"name":"entry_point_name","type":{"List":{"Option":"U256"}},"value":[1,null,"3"]}"#,
                r#"{"name":"entry_point_name","type":{"List":"U8"},"value":"0102ff"}"#,
            ],
        },
        InfoAndExamples {
            info:
                "CLType ByteArray is represented as a JSON String (hex-encoded) or more verbosely \
                by an Array of Numbers, e.g.",
            examples: vec![
                r#"{"name":"entry_point_name","type":{"ByteArray":3},"value":"0114ff"}"#,
                r#"{"name":"entry_point_name","type":{"ByteArray":3},"value":[1,20,255]}"#,
            ],
        },
        InfoAndExamples {
            info:
                "CLType Result<T, E> is represented as a JSON Object with exactly one entry named \
                either \"Ok\" or \"Err\" where the Object's value is suitable to represent T or E \
                respectively, e.g.",
            examples: vec![
                r#"{"name":"entry_point_name","type":{"Result":{"ok":"Bool","err":"U8"}},"value":{"Ok":true}}"#,
                r#"{"name":"entry_point_name","type":{"Result":{"ok":"Bool","err":"U8"}},"value":{"Err":1}}"#,
            ],
        },
        InfoAndExamples {
            info: "CLType Map<K, V> is represented as a JSON Array of Objects of the form \
                {\"key\":<K-VALUE>,\"value\":<V-VALUE>}.  For the special case where K is String \
                or a numerical type, the Map can be represented as a single JSON Object, with each \
                entry having the name of the given key as a String, e.g.",
            examples: vec![
                r#"{"name":"entry_point_name","type":{"Map":{"key":"U8","value":"Bool"}},"value":[{"key":1,"value":true},{"key":2,"value":false}]}"#,
                r#"{"name":"entry_point_name","type":{"Map":{"key":"U8","value":"Bool"}},"value":{"1":true,"2":false}}"#,
            ],
        },
        InfoAndExamples {
            info: "CLTypes Tuple1, Tuple2 and Tuple3 are represented as a JSON Array, e.g.",
            examples: vec![
                r#"{"name":"entry_point_name","type":{"Tuple1":["Bool"]},"value":[true]}"#,
                r#"{"name":"entry_point_name","type":{"Tuple2":["Bool","U8"]},"value":[true,128]}"#,
                r#"{"name":"entry_point_name","type":{"Tuple3":["Bool","U8","String"]},"value":[true,128,"a"]}"#,
            ],
        },
    ]
});

static HELP: Lazy<String> = Lazy::new(|| {
    let mut help = r#"The value must be a JSON Array of JSON Objects of the form
{"name":<String>,"type":<VALUE>,"value":<VALUE>}

For example, to provide the following session args:
* The value "square" to an entry point named "shape" taking a CLType::String
* The tuple value (100,100) to an entry point named "dimensions" taking a CLType::Tuple2<CLType::U32, CLType::U32>
* The value "blue" to an entry point named "color" taking a CLType::Option<CLType::String>
the following input would be used:
'[{"name":"shape","type":"String","value":"square"},{"name":"dimensions","type":{"Tuple2":["U32","U32"]},"value":[100,100]},{"name":"color","type":{"Option":"String"},"value":"blue"}]'

Details for each CLType variant:
"#.to_string();

    ALL_INFO_AND_EXAMPLES.iter().for_each(|elt| {
        let mut line_number = 0;
        let mut current_line_length = 0;
        for word in elt.info.split_whitespace() {
            if current_line_length + word.len() + 1 > MAX_INFO_LINE_LENGTH
                && current_line_length != 0
            {
                help += "\n";
                line_number += 1;
                current_line_length = 0;
            }
            if current_line_length == 0 {
                let bullet_or_space = if line_number == 0 { '*' } else { ' ' };
                let _ = write!(help, "{} {}", bullet_or_space, word);
                current_line_length += word.len() + 2;
            } else {
                let _ = write!(help, " {}", word);
                current_line_length += word.len() + 1;
            }
        }

        help += "\n";

        elt.examples.iter().for_each(|example| {
            let _ = writeln!(help, "  {}", example);
        });
        help += "\n";
    });

    help += "Note that CLType Any cannot be represented as JSON.\n";
    help
});

/// Returns a string listing examples of the format required when passing in payment code or
/// session code args.
pub fn info_and_examples() -> &'static str {
    &HELP
}

#[cfg(test)]
mod tests {
    use casper_types::NamedArg;

    use super::*;
    use crate::cli::json_args::JsonArg;

    #[test]
    fn should_parse_examples() {
        for example in ALL_INFO_AND_EXAMPLES.iter().flat_map(|elt| &elt.examples) {
            let parsed: JsonArg = serde_json::from_str(example).unwrap();
            NamedArg::try_from(parsed)
                .unwrap_or_else(|error| panic!("unexpected error: {}", error));
        }
    }
}
