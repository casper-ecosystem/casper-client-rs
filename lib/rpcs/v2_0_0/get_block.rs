use serde::{Deserialize, Serialize};

use casper_types::ProtocolVersion;

pub(crate) use crate::rpcs::v1_6_0::get_block::{GetBlockParams, GET_BLOCK_METHOD};

use crate::types::JsonBlockWithSignatures;

/// The `result` field of a successful JSON-RPC response to a `chain_get_block` request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetBlockResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// The block, if found.
    pub block_with_signatures: Option<JsonBlockWithSignatures>,
}
