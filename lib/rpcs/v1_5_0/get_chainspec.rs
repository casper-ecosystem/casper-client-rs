use serde::{Deserialize, Serialize};

use casper_types::{ChainspecRawBytes, ProtocolVersion};

pub(crate) const GET_CHAINSPEC_METHOD: &str = "info_get_chainspec";

/// The `result` field of a successful JSON-RPC response to a `info_get_chainspec` request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetChainspecResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// The chainspec file bytes.
    pub chainspec_bytes: ChainspecRawBytes,
}
