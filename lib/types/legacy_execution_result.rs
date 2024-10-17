use serde::{Deserialize, Serialize};

use casper_types::BlockHash;

/// The execution result of a single deploy.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct LegacyExecutionResult {
    /// The block hash.
    pub block_hash: BlockHash,
    /// Execution result.
    pub result: casper_types::execution::ExecutionResultV1,
}
