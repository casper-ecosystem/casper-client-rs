use serde::{Deserialize, Serialize};

use casper_types::{execution::ExecutionResult, BlockHash};

/// The block hash and height in which a given deploy was executed, along with the execution result
/// if known.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DeployExecutionInfo {
    /// The block hash in which the deploy was executed.
    pub block_hash: BlockHash,
    /// The block height in which the deploy was executed.
    pub block_height: u64,
    /// The execution result if known.
    pub execution_result: Option<ExecutionResult>,
}
