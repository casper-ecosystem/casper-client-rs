use serde::{Deserialize, Serialize};

use casper_types::ProtocolVersion;

use crate::rpcs::{common::BlockIdentifier, results::EraSummary};

pub(crate) const GET_ERA_SUMMARY_METHOD: &str = "chain_get_era_summary";

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct GetEraSummaryParams {
    block_identifier: BlockIdentifier,
}

impl GetEraSummaryParams {
    pub(crate) fn new(block_identifier: BlockIdentifier) -> Self {
        GetEraSummaryParams { block_identifier }
    }
}

/// The `result` field of a successful JSON-RPC response to a `chain_get_era_summary` request.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct GetEraSummaryResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// The era summary.
    pub era_summary: EraSummary,
}
