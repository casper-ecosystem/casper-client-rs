//! Common types associated with sending and receiving JSON-RPCs.

use serde::{Deserialize, Serialize};

#[cfg(doc)]
use casper_types::Block;
use casper_types::{
    contract_messages::Messages, execution::Effects, BlockHash, Digest, Gas, Transfer,
};

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

/// The result of a speculative execution.
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct SpeculativeExecutionResult {
    /// Block hash against which the execution was performed.
    pub block_hash: BlockHash,
    /// List of transfers that happened during execution.
    pub transfers: Vec<Transfer>,
    /// Gas limit.
    pub limit: Gas,
    /// Gas consumed.
    pub consumed: Gas,
    /// Execution effects.
    pub effects: Effects,
    /// Messages emitted during execution.
    pub messages: Messages,
    /// Did the wasm execute successfully?
    pub error: Option<String>,
}
