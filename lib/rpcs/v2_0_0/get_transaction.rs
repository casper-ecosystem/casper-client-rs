use serde::{Deserialize, Serialize};

use casper_types::execution::ExecutionResult;
use casper_types::{BlockHash, ProtocolVersion, Transaction, TransactionHash};

pub(crate) const GET_TRANSACTION_METHOD: &str = "info_get_transaction";

/// Params for "info_get_transaction" RPC request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetTransactionParams {
    /// The transaction hash.
    pub transaction_hash: TransactionHash,
    /// Whether to return the transaction with the finalized approvals substituted. If `false` or
    /// omitted, returns the transaction with the approvals that were originally received by the
    /// node.
    #[serde(default = "finalized_approvals_default")]
    pub finalized_approvals: bool,
}

/// The default for `GetDeployParams::finalized_approvals` and
/// `GetTransactionParams::finalized_approvals`.
fn finalized_approvals_default() -> bool {
    false
}

impl GetTransactionParams {
    pub(crate) fn new(transaction_hash: TransactionHash, finalized_approvals: bool) -> Self {
        GetTransactionParams {
            transaction_hash,
            finalized_approvals,
        }
    }
}

/// The block hash and height in which a given deploy was executed, along with the execution result
/// if known.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ExecutionInfo {
    pub(crate) block_hash: BlockHash,
    pub(crate) block_height: u64,
    pub(crate) execution_result: Option<ExecutionResult>,
}

/// Result for "info_get_transaction" RPC response.
#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetTransactionResult {
    /// The RPC API version.
    pub api_version: ProtocolVersion,
    /// The transaction.
    pub transaction: Transaction,
    /// Execution info, if available.
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    pub execution_info: Option<ExecutionInfo>,
}
