use serde::{Deserialize, Serialize};

use crate::rpcs::common::{BlockIdentifier, PurseIdentifier};

pub use crate::rpcs::v1_4_5::get_account::GetAccountResult;

pub(crate) const GET_ACCOUNT_METHOD: &str = "state_get_account_info";

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct GetAccountParams {
    purse_identifier: PurseIdentifier,
    block_identifier: Option<BlockIdentifier>,
}

impl GetAccountParams {
    pub(crate) fn new(purse_identifier: PurseIdentifier, block_identifier: Option<BlockIdentifier>) -> Self {
        GetAccountParams {
            purse_identifier,
            block_identifier,
        }
    }
}

