use serde::{Deserialize, Serialize};

use casper_types::{BlockTime, ProtocolVersion, U512};

use crate::rpcs::{GlobalStateIdentifier, PurseIdentifier};

pub(crate) const QUERY_BALANCE_DETAILS_METHOD: &str = "query_balance_details";

/// Params for "query_balance_details" RPC request.
#[derive(Serialize, Deserialize, Debug)]
pub struct QueryBalanceDetailsParams {
    /// The identifier for the state used for the query, if none is passed,
    /// the latest block will be used.
    pub state_identifier: Option<GlobalStateIdentifier>,
    /// The identifier to obtain the purse corresponding to balance query.
    pub purse_identifier: PurseIdentifier,
}

impl QueryBalanceDetailsParams {
    pub(crate) fn new(
        state_identifier: Option<GlobalStateIdentifier>,
        purse_identifier: PurseIdentifier,
    ) -> Self {
        QueryBalanceDetailsParams {
            state_identifier,
            purse_identifier,
        }
    }
}
/// Result for "query_balance_details" RPC response.
#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct QueryBalanceDetailsResult {
    /// The RPC API version.
    pub api_version: ProtocolVersion,
    /// The purses total balance, not considering holds.
    pub total_balance: U512,
    /// The available balance in motes (total balance - sum of all active holds).
    pub available_balance: U512,
    /// A proof that the given value is present in the Merkle trie.
    pub total_balance_proof: String,
    /// Holds active at the requested point in time.
    pub holds: Vec<BalanceHoldWithProof>,
}

/// A hold on an account's balance.
#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct BalanceHoldWithProof {
    /// The block time at which the hold was created.
    pub time: BlockTime,
    /// The amount in the hold.
    pub amount: U512,
    /// A proof that the given value is present in the Merkle trie.
    pub proof: String,
}
