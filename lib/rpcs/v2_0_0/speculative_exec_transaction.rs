use serde::{Deserialize, Serialize};

use casper_types::{
    contract_messages::Messages, execution::ExecutionResultV2, BlockHash, ProtocolVersion,
    Transaction,
};

use crate::rpcs::common::BlockIdentifier;

pub(crate) const SPECULATIVE_EXEC_TXN_METHOD: &str = "speculative_exec_txn";

/// Params for "speculative_exec_txn" RPC request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SpeculativeExecTxnParams {
    /// Block hash on top of which to execute the transaction.
    pub block_identifier: Option<BlockIdentifier>,
    /// Transaction to execute.
    pub transaction: Transaction,
}

impl SpeculativeExecTxnParams {
    /// Creates a new `SpeculativeExecTxnParams`.
    pub fn new(block_identifier: Option<BlockIdentifier>, transaction: Transaction) -> Self {
        SpeculativeExecTxnParams {
            block_identifier,
            transaction,
        }
    }
}

/// Result for "speculative_exec_txn" and "speculative_exec" RPC responses.
#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SpeculativeExecTxnResult {
    /// The RPC API version.
    pub api_version: ProtocolVersion,
    /// Hash of the block on top of which the transaction was executed.
    pub block_hash: BlockHash,
    /// Result of the execution.
    pub execution_result: ExecutionResultV2,
    /// Messages emitted during execution.
    pub messages: Messages,
}
