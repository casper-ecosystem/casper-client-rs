use serde::{Deserialize, Serialize};

use casper_types::{ProtocolVersion, PublicKey};

use crate::{rpcs::common::BlockIdentifier, types::Account};

pub(crate) const GET_ACCOUNT_METHOD: &str = "state_get_account_info";

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct GetAccountParams {
    ///The identifier of an Account. (named public key to match the JSON-RPC API)
    public_key: PublicKey,
    /// The block identifier.
    block_identifier: Option<BlockIdentifier>,
}

impl GetAccountParams {
    //This clippy lint should be re-enabled once the client is updated to handle multiple different node versions.
    #[allow(dead_code)]
    pub(crate) fn new(public_key: PublicKey, block_identifier: Option<BlockIdentifier>) -> Self {
        GetAccountParams {
            public_key,
            block_identifier,
        }
    }
}

/// The `result` field of a successful JSON-RPC response to a `state_get_account_info` request.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct GetAccountResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// The account.
    pub account: Account,
    /// The merkle proof of the value.
    pub merkle_proof: String,
}
