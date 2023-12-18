use serde::{Deserialize, Serialize};

use super::BlockHash;

/// The execution result of a single deploy.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ExecutionResult {
    /// The block hash.
    pub block_hash: BlockHash,
    /// Execution result.
    pub result: casper_types::ExecutionResult,
}
