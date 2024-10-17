use serde::{Deserialize, Serialize};

use casper_types::{Deploy, ProtocolVersion};

use crate::rpcs::common::SpeculativeExecutionResult;
pub(crate) use crate::rpcs::v1_6_0::speculative_exec::SPECULATIVE_EXEC_METHOD;

/// Params for "speculative_exec" RPC request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SpeculativeExecParams {
    /// Deploy to execute.
    pub deploy: Deploy,
}

impl SpeculativeExecParams {
    /// Creates a new `SpeculativeExecParams`.
    pub fn new(deploy: Deploy) -> Self {
        SpeculativeExecParams { deploy }
    }
}

/// Result for "speculative_exec_txn" and "speculative_exec" RPC responses.
#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SpeculativeExecResult {
    /// The RPC API version.
    pub api_version: ProtocolVersion,
    /// Result of the speculative execution.
    pub execution_result: SpeculativeExecutionResult,
}
