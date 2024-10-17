pub(crate) use crate::rpcs::v1_4_5::get_node_status::GET_NODE_STATUS_METHOD;
pub use crate::rpcs::v1_4_5::get_node_status::{ActivationPoint, MinimalBlockInfo, NextUpgrade};

use serde::{Deserialize, Serialize};

use casper_types::{BlockHash, Digest, ProtocolVersion, PublicKey, TimeDiff, Timestamp};

use super::get_peers::PeerEntry;

/// The state of the reactor.
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum ReactorState {
    /// Get all components and reactor state set up on start.
    Initialize,
    /// Orient to the network and attempt to catch up to tip.
    CatchUp,
    /// Running commit upgrade and creating immediate switch block.
    Upgrading,
    /// Stay caught up with tip.
    KeepUp,
    /// Node is currently caught up and is an active validator.
    Validate,
    /// Node should be shut down for upgrade.
    ShutdownForUpgrade,
}

/// An unbroken, inclusive range of blocks.
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct AvailableBlockRange {
    /// The inclusive lower bound of the range.
    low: u64,
    /// The inclusive upper bound of the range.
    high: u64,
}

/// The status of syncing an individual block.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct BlockSyncStatus {
    /// The block hash.
    block_hash: BlockHash,
    /// The height of the block, if known.
    block_height: Option<u64>,
    /// The state of acquisition of the data associated with the block.
    acquisition_state: String,
}

/// The status of the block synchronizer.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct BlockSynchronizerStatus {
    /// The status of syncing a historical block, if any.
    historical: Option<BlockSyncStatus>,
    /// The status of syncing a forward block, if any.
    forward: Option<BlockSyncStatus>,
}

/// The `result` field of a successful JSON-RPC response to a `info_get_status` request.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct GetNodeStatusResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// The chainspec name.
    pub chainspec_name: String,
    /// The state root hash used at the start of the current session.
    #[deprecated(since = "1.5.0")]
    pub starting_state_root_hash: Digest,
    /// The node ID and network address of each connected peer.
    pub peers: Vec<PeerEntry>,
    /// The minimal info of the last block from the linear chain.
    pub last_added_block_info: Option<MinimalBlockInfo>,
    /// The node's public signing key.
    pub our_public_signing_key: Option<PublicKey>,
    /// The next round length if this node is a validator.
    pub round_length: Option<TimeDiff>,
    /// Information about the next scheduled upgrade staged for this node.
    pub next_upgrade: Option<NextUpgrade>,
    /// The compiled node version.
    pub build_version: String,
    /// Time that passed since the node has started.
    pub uptime: TimeDiff,
    /// The current state of node reactor.
    pub reactor_state: ReactorState,
    /// Timestamp of the last recorded progress in the reactor.
    pub last_progress: Timestamp,
    /// The available block range in storage.
    pub available_block_range: AvailableBlockRange,
    /// The status of the block synchronizer builders.
    pub block_sync: BlockSynchronizerStatus,
}
