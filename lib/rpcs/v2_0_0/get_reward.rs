use casper_types::{BlockHash, EraId, ProtocolVersion, PublicKey, U512};
use serde::{Deserialize, Serialize};

use crate::rpcs::common::BlockIdentifier;

pub(crate) const GET_REWARD_METHOD: &str = "info_get_reward";

/// Identifier for an era.
#[derive(Serialize, Deserialize, Debug)]
pub enum EraIdentifier {
    /// An era identifier.
    Era(EraId),
    /// A block identifier.
    Block(BlockIdentifier),
}

/// Params for "info_get_reward" RPC request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetRewardParams {
    /// The era identifier. If `None`, the last finalized era is used.
    pub era_identifier: Option<EraIdentifier>,
    /// The public key of the validator.
    pub validator: PublicKey,
    /// The public key of the delegator. If `Some`, the rewards for the delegator are returned.
    /// If `None`, the rewards for the validator are returned.
    pub delegator: Option<PublicKey>,
}

impl GetRewardParams {
    pub fn new(
        era_identifier: Option<EraIdentifier>,
        validator: PublicKey,
        delegator: Option<PublicKey>,
    ) -> Self {
        Self {
            era_identifier,
            validator,
            delegator,
        }
    }
}

/// Result for "info_get_reward" RPC response.
#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetRewardResult {
    /// The RPC API version.
    pub api_version: ProtocolVersion,
    /// The total reward amount in the requested era.
    pub reward_amount: U512,
    /// The era for which the reward was calculated.
    pub era_id: EraId,
    /// The delegation rate of the validator.
    pub delegation_rate: u8,
    /// The switch block hash at which the reward was distributed.
    pub switch_block_hash: BlockHash,
}
