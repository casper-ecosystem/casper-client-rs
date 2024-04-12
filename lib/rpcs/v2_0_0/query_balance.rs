use serde::{Deserialize, Serialize};

use casper_types::{account::AccountHash, EntityAddr, PublicKey, URef};

use crate::rpcs::common::GlobalStateIdentifier;

pub use crate::rpcs::v1_5_0::query_balance::QueryBalanceResult;
pub(crate) use crate::rpcs::v1_5_0::query_balance::QUERY_BALANCE_METHOD;

/// Identifier of a purse.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum PurseIdentifier {
    /// The main purse of the account identified by this public key.
    MainPurseUnderPublicKey(PublicKey),
    /// The main purse of the account identified by this account hash.
    MainPurseUnderAccountHash(AccountHash),
    /// The main purse of the account identified by this entity address.
    MainPurseUnderEntityAddr(EntityAddr),
    /// The purse identified by this URef.
    PurseUref(URef),
}

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
