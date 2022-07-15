pub(crate) use crate::rpcs::v1_4_5::get_node_status::GET_NODE_STATUS_METHOD;
pub use crate::rpcs::v1_4_5::get_node_status::{ActivationPoint, MinimalBlockInfo, NextUpgrade};

use serde::{Deserialize, Serialize};

use casper_hashing::Digest;
use casper_types::{ProtocolVersion, PublicKey};

use super::get_peers::PeerEntry;
use crate::types::{BlockHash, TimeDiff};

/// The reason for syncing the trie store under a given state root hash.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum FetchingTriesReason {
    /// Performing a trie-store sync during fast-sync before moving to the "execute forwards"
    /// phase.
    #[serde(rename = "for fast sync")]
    FastSync,
    /// Performing a trie-store sync during fast-sync before performing an emergency upgrade.
    #[serde(rename = "preparing for emergency upgrade")]
    EmergencyUpgrade,
    /// Performing a trie-store sync during fast-sync before performing an upgrade.
    #[serde(rename = "preparing for upgrade")]
    Upgrade,
}

/// The progress of the fast-sync task, performed by all nodes while in joining mode.
///
/// The fast-sync task generally progresses from each variant to the next linearly.  An exception is
/// that it will cycle repeatedly from `FetchingBlockAndDeploysToExecute` to `ExecutingBlock` (and
/// possibly to `RetryingBlockExecution`) as it iterates forwards one block at a time, fetching then
/// executing.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum FastSync {
    /// Fast-syncing has not started yet.
    #[serde(rename = "not yet started")]
    NotYetStarted,
    /// Initial setup is being performed.
    Starting,
    /// Currently fetching the trusted block header.
    FetchingTrustedBlockHeader(BlockHash),
    /// Currently syncing the trie-store under the given state root hash.
    #[serde(rename = "fetching_global_state_under_given_block")]
    FetchingTries {
        /// The height of the block containing the given state root hash.
        block_height: u64,
        /// The global state root hash.
        state_root_hash: Digest,
        /// The reason for syncing the trie-store.
        reason: FetchingTriesReason,
        /// The number of remaining tries to fetch (this value can rise and fall as the task
        /// proceeds).
        #[serde(rename = "number_of_remaining_tries_to_fetch")]
        num_tries_to_fetch: usize,
    },
    /// Currently fetching the block at the given height and its deploys in preparation for
    /// executing it.
    #[serde(rename = "fetching_block_and_deploys_at_given_block_height_before_execution")]
    FetchingBlockAndDeploysToExecute(u64),
    /// Currently executing the block at the given height.
    #[serde(rename = "executing_block_at_height")]
    ExecutingBlock(u64),
    /// Currently retrying the execution of the block at the given height, due to an unexpected
    /// mismatch in the previous execution result (due to a mismatch in the deploys' approvals).
    RetryingBlockExecution {
        /// The height of the block being executed.
        block_height: u64,
        /// The retry attempt number.
        attempt: usize,
    },
    /// Fast-syncing has finished, and the node will shortly transition to participating mode.
    Finished,
}

/// The progress of a single sync-block task, many of which are performed in parallel during
/// sync-to-genesis.
///
/// The task progresses from each variant to the next linearly.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum SyncBlockFetching {
    /// Currently fetching the block and all its deploys.
    #[serde(rename = "block and deploys")]
    BlockAndDeploys,
    /// Currently syncing the trie-store under the state root hash held in the block.
    #[serde(rename = "global_state_under_given_block")]
    Tries {
        /// The global state root hash.
        state_root_hash: Digest,
        /// The number of remaining tries to fetch (this value can rise and fall as the task
        /// proceeds).
        #[serde(rename = "number_of_remaining_tries_to_fetch")]
        num_tries_to_fetch: usize,
    },
    /// Currently fetching the block's signatures.
    #[serde(rename = "block signatures")]
    BlockSignatures,
}

/// Container pairing the given [`SyncBlockFetching`] progress indicator with the height of the
/// relative block.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct SyncBlock {
    /// The height of the block being synced.
    block_height: u64,
    /// The progress of the sync-block task.
    fetching: SyncBlockFetching,
}

/// The progress of the sync-to-genesis task, only performed by nodes configured to do this, and
/// performed by them while in participating mode.
///
/// The sync-to-genesis task progresses from each variant to the next linearly.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SyncToGenesis {
    /// Syncing-to-genesis has not started yet.
    #[serde(rename = "not yet started")]
    NotYetStarted,
    /// Initial setup is being performed.
    Starting,
    /// Currently fetching block headers in batches back towards the genesis block.
    FetchingHeadersBackToGenesis {
        /// The current lowest block header retrieved by this fetch-headers-to-genesis task.
        lowest_block_height: u64,
    },
    /// Currently syncing all blocks from genesis towards the tip of the chain.
    ///
    /// This is done via many parallel sync-block tasks, with each such ongoing task being
    /// represented by an entry in the wrapped `Vec`.  The set is sorted ascending by block height.
    SyncingForwardFromGenesis(Vec<SyncBlock>),
    /// Syncing-to-genesis has finished.
    Finished,
}

/// The progress of the chain-synchronizer task.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Progress {
    /// The chain-synchronizer is performing the fast-sync task.
    FastSync(FastSync),
    /// The chain-synchronizer is performing the sync-to-genesis task.
    SyncToGenesis(SyncToGenesis),
}

/// The various possible states of operation for the node.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum NodeState {
    /// The node is currently in joining mode.
    Joining(Progress),
    /// The node is currently in participating mode, but also syncing to genesis in the background.
    ParticipatingAndSyncingToGenesis {
        /// The progress of the chain sync-to-genesis task.
        sync_progress: Progress,
    },
    /// The node is currently in the participating state.
    Participating,
}

/// The `result` field of a successful JSON-RPC response to a `info_get_status` request.
#[derive(Serialize, Deserialize, Debug)]
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
    /// The current state of node.
    pub node_state: NodeState,
}
