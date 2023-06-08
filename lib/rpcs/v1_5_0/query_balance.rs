use serde::{Deserialize, Serialize};

use casper_types::{ProtocolVersion, U512};

use crate::rpcs::common::{GlobalStateIdentifier, PurseIdentifier};

pub(crate) const QUERY_BALANCE_METHOD: &str = "query_balance";

/// Params for "query_balance" RPC request.
#[derive(Serialize, Deserialize, Debug)]
pub struct QueryBalanceParams {
    /// The state identifier used for the query.
    pub state_identifier: Option<GlobalStateIdentifier>,
    /// The identifier to obtain the purse corresponding to balance query.
    pub purse_identifier: PurseIdentifier,
}

impl QueryBalanceParams {
    pub(crate) fn new(
        state_identifier: Option<GlobalStateIdentifier>,
        purse_identifier: PurseIdentifier,
    ) -> Self {
        QueryBalanceParams {
            state_identifier,
            purse_identifier,
        }
    }
}

/// Result for "query_balance" RPC response.
#[derive(Serialize, Deserialize, Debug)]
pub struct QueryBalanceResult {
    /// The RPC API version.
    pub api_version: ProtocolVersion,
    /// The balance represented in motes.
    pub balance: U512,
}
