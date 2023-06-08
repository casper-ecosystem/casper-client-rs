use serde::{Deserialize, Serialize};

use casper_types::{ProtocolVersion, PublicKey};

use crate::{rpcs::common::BlockIdentifier, types::Account};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct GetAccountParams {
    public_key: PublicKey,
    block_identifier: Option<BlockIdentifier>,
}

/// The `result` field of a successful JSON-RPC response to a `state_get_account_info` request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetAccountResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// The account.
    pub account: Account,
    /// The merkle proof of the value.
    pub merkle_proof: String,
}
