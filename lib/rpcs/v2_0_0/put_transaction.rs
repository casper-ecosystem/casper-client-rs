use serde::{Deserialize, Serialize};

use casper_types::{ProtocolVersion, Transaction, TransactionHash};

pub(crate) const PUT_TRANSACTION_METHOD: &str = "account_put_transaction";

/// Params for "account_put_transaction" RPC request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PutTransactionParams {
    /// The `Transaction`.
    pub transaction: Transaction,
}

impl PutTransactionParams {
    pub(crate) fn new(transaction: Transaction) -> Self {
        PutTransactionParams { transaction }
    }
}

/// Result for "account_put_transaction" RPC response.
#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PutTransactionResult {
    /// The RPC API version.
    pub api_version: ProtocolVersion,
    /// The transaction hash.
    pub transaction_hash: TransactionHash,
}
