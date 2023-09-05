use serde::{Deserialize, Serialize};

use casper_types::{Key, ProtocolVersion};

pub(crate) use crate::rpcs::v1_4_5::query_global_state::QUERY_GLOBAL_STATE_METHOD;
use crate::{
    rpcs::common::GlobalStateIdentifier,
    types::{BlockHeader, StoredValue},
};

#[cfg(doc)]
use crate::BlockIdentifier;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct QueryGlobalStateParams {
    /// The identifier used for the query. If none is passed
    /// the tip of the chain will be used.
    state_identifier: Option<GlobalStateIdentifier>,
    /// `casper_types::Key` as formatted string.
    key: String,
    /// The path components starting from the key as base.
    path: Vec<String>,
}

impl QueryGlobalStateParams {
    pub(crate) fn new(
        state_identifier: Option<GlobalStateIdentifier>,
        key: Key,
        path: Vec<String>,
    ) -> Self {
        QueryGlobalStateParams {
            state_identifier,
            key: key.to_formatted_string(),
            path,
        }
    }
}

/// The `result` field of a successful JSON-RPC response to a `query_global_state` request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct QueryGlobalStateResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// The block header if the query was made using a block hash.
    pub block_header: Option<BlockHeader>,
    /// The stored value.
    pub stored_value: StoredValue,
    /// The merkle proof of the value.
    pub merkle_proof: String,
}
