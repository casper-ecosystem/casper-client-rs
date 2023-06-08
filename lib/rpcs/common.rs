//! Common types associated with sending and receiving JSON-RPCs.

#[cfg(doc)]
use crate::types::Block;
use crate::types::BlockHash;

use serde::{Deserialize, Serialize};

use casper_hashing::Digest;
use casper_types::{account::AccountHash, PublicKey, URef};

/// Enum of possible ways to identify a [`Block`].
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum BlockIdentifier {
    /// Identify the block by its hash.
    Hash(BlockHash),
    /// Identify the block by its height.
    Height(u64),
}

/// Identifier for possible ways to query global state.
///
/// Soon to be deprecated in favour of [`BlockIdentifier`].
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum GlobalStateIdentifier {
    /// Query using the state root hash in the given block, identified by its block hash.
    BlockHash(BlockHash),
    /// Query using the state root hash in the given block, identified by its block height.
    BlockHeight(u64),
    /// Query using the state root hash.
    StateRootHash(Digest),
}

/// Identifier of a purse.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum PurseIdentifier {
    /// The main purse of the account identified by this public key.
    MainPurseUnderPublicKey(PublicKey),
    /// The main purse of the account identified by this account hash.
    MainPurseUnderAccountHash(AccountHash),
    /// The purse identified by this URef.
    PurseUref(URef),
}

