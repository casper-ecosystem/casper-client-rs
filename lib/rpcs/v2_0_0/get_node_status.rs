use serde::{Deserialize, Serialize};

use casper_types::{
    BlockHash, BlockSynchronizerStatus, Digest, Peers, ProtocolVersion, PublicKey, TimeDiff,
    Timestamp,
};

pub(crate) use crate::rpcs::v1_6_0::get_node_status::GET_NODE_STATUS_METHOD;

pub use crate::rpcs::v1_6_0::get_node_status::{
    AvailableBlockRange, MinimalBlockInfo, NextUpgrade,
};

/// Result for "info_get_status" RPC response.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetNodeStatusResult {
    /// The RPC API version.
    pub api_version: ProtocolVersion,
    /// The node ID and network address of each connected peer.
    pub peers: Peers,
    /// The compiled node version.
    pub build_version: String,
    /// The chainspec name.
    pub chainspec_name: String,
    /// The state root hash of the lowest block in the available block range.
    pub starting_state_root_hash: Digest,
    /// The minimal info of the last block from the linear chain.
    pub last_added_block_info: Option<MinimalBlockInfo>,
    /// Our public signing key.
    pub our_public_signing_key: Option<PublicKey>,
    /// The next round length if this node is a validator.
    pub round_length: Option<TimeDiff>,
    /// Information about the next scheduled upgrade.
    pub next_upgrade: Option<NextUpgrade>,
    /// Time that passed since the node has started.
    pub uptime: TimeDiff,
    /// The name of the current state of node reactor.
    pub reactor_state: String,
    /// Timestamp of the last recorded progress in the reactor.
    pub last_progress: Timestamp,
    /// The available block range in storage.
    pub available_block_range: AvailableBlockRange,
    /// The status of the block synchronizer builders.
    pub block_sync: BlockSynchronizerStatus,
    /// The hash of the latest switch block.
    pub latest_switch_block_hash: Option<BlockHash>,
}
