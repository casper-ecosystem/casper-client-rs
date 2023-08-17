use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use casper_types::account::AccountHash;
use casper_types::{ProtocolVersion, PublicKey};

use crate::{rpcs::common::BlockIdentifier, types::Account};

pub(crate) const GET_ACCOUNT_METHOD: &str = "state_get_account_info";

/// Identifier of an account.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(deny_unknown_fields, untagged)]
pub enum AccountIdentifier {
    /// The public key of an account
    PublicKey(PublicKey),
    /// The account hash of an account
    AccountHash(AccountHash),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct GetAccountParams {
    ///The identifier of an Account. (named public key to match the JSON-RPC API)
    public_key: AccountIdentifier,
    /// The block identifier.
    block_identifier: Option<BlockIdentifier>,
}

impl GetAccountParams {
    pub(crate) fn new(
        account_identifier: AccountIdentifier,
        block_identifier: Option<BlockIdentifier>,
    ) -> Self {
        GetAccountParams {
            public_key: account_identifier,
            block_identifier,
        }
    }
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
