use serde::{Deserialize, Serialize};

use casper_types::ProtocolVersion;

use crate::rpcs::common::BlockIdentifier;

use crate::types::{Deploy, DeployHash};

pub(crate) const SPECULATIVE_EXEC_METHOD: &str = "speculative_exec";

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct SpeculativeExecParams {
    block_identifier: Option<BlockIdentifier>,
    deploy: Deploy,
}

impl SpeculativeExecParams {
    pub(crate) fn new(block_identifier: Option<BlockIdentifier>, deploy: Deploy) -> Self {
        SpeculativeExecParams {
            block_identifier,
            deploy,
        }
    }
}

/// The `result` field of a successful JSON-RPC response to an `speculative_exec` request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SpeculativeExecResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// The deploy hash.
    pub deploy_hash: DeployHash,
}
