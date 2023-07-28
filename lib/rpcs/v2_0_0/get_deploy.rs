use serde::{Deserialize, Serialize};

use casper_types::{execution::ExecutionResult, BlockHash, Deploy, ProtocolVersion};

pub(crate) use crate::rpcs::v1_4_5::get_deploy::{GetDeployParams, GET_DEPLOY_METHOD};

/// The block hash and height in which a given deploy was executed, along with the execution result
/// if known.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DeployExecutionInfo {
    pub(crate) block_hash: BlockHash,
    pub(crate) block_height: u64,
    pub(crate) execution_result: Option<ExecutionResult>,
}

/// The `result` field of a successful JSON-RPC response to an `info_get_deploy` request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetDeployResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// The deploy.
    pub deploy: Deploy,
    /// Execution info, if available.
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    pub execution_info: Option<DeployExecutionInfo>,
}
