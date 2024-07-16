//! The JSON-RPC request and response types at v1.5.0 of casper-node.

pub(crate) mod get_account;

pub(crate) mod query_global_state;

// The following RPCs are all unchanged from v1.5.0, so we just re-export them.

pub(crate) mod get_chainspec {
    pub(crate) use crate::rpcs::v1_5_0::get_chainspec::GET_CHAINSPEC_METHOD;
    pub use crate::rpcs::v1_5_0::get_chainspec::{ChainspecRawBytes, GetChainspecResult};
}

pub(crate) mod get_deploy {
    pub use crate::rpcs::v1_5_0::get_deploy::GetDeployResult;
    pub(crate) use crate::rpcs::v1_5_0::get_deploy::{GetDeployParams, GET_DEPLOY_METHOD};
}

pub(crate) mod get_era_summary {
    pub use crate::rpcs::v1_5_0::get_era_summary::GetEraSummaryResult;
    pub(crate) use crate::rpcs::v1_5_0::get_era_summary::{
        GetEraSummaryParams, GET_ERA_SUMMARY_METHOD,
    };
}

pub(crate) mod get_node_status {
    pub(crate) use crate::rpcs::v1_5_0::get_node_status::GET_NODE_STATUS_METHOD;
    pub use crate::rpcs::v1_5_0::get_node_status::{
        ActivationPoint, AvailableBlockRange, GetNodeStatusResult, MinimalBlockInfo, NextUpgrade,
        ReactorState,
    };
}

pub(crate) mod query_balance {
    pub use crate::rpcs::v1_5_0::query_balance::QueryBalanceResult;
    pub(crate) use crate::rpcs::v1_5_0::query_balance::{
        PurseIdentifier, QueryBalanceParams, QUERY_BALANCE_METHOD,
    };
}

pub(crate) mod speculative_exec {
    pub use crate::rpcs::v1_5_0::speculative_exec::SpeculativeExecResult;
    pub(crate) use crate::rpcs::v1_5_0::speculative_exec::{
        SpeculativeExecParams, SPECULATIVE_EXEC_METHOD,
    };
}

pub(crate) mod get_auction_info {
    pub use crate::rpcs::v1_5_0::get_auction_info::GetAuctionInfoResult;
    pub(crate) use crate::rpcs::v1_5_0::get_auction_info::{
        GetAuctionInfoParams, GET_AUCTION_INFO_METHOD,
    };
}

pub(crate) mod get_balance {
    pub use crate::rpcs::v1_5_0::get_balance::GetBalanceResult;
    pub(crate) use crate::rpcs::v1_5_0::get_balance::{GetBalanceParams, GET_BALANCE_METHOD};
}

pub(crate) mod get_block {
    pub use crate::rpcs::v1_5_0::get_block::GetBlockResult;
    pub(crate) use crate::rpcs::v1_5_0::get_block::{GetBlockParams, GET_BLOCK_METHOD};
}

pub(crate) mod get_block_transfers {
    pub use crate::rpcs::v1_5_0::get_block_transfers::GetBlockTransfersResult;
    pub(crate) use crate::rpcs::v1_5_0::get_block_transfers::{
        GetBlockTransfersParams, GET_BLOCK_TRANSFERS_METHOD,
    };
}

pub(crate) mod get_dictionary_item {
    pub use crate::rpcs::v1_5_0::get_dictionary_item::GetDictionaryItemResult;
    pub(crate) use crate::rpcs::v1_5_0::get_dictionary_item::{
        GetDictionaryItemParams, GET_DICTIONARY_ITEM_METHOD,
    };
}

pub(crate) mod get_era_info {
    pub use crate::rpcs::v1_5_0::get_era_info::{EraSummary, GetEraInfoResult};
    pub(crate) use crate::rpcs::v1_5_0::get_era_info::{GetEraInfoParams, GET_ERA_INFO_METHOD};
}

pub(crate) mod get_peers {
    pub(crate) use crate::rpcs::v1_5_0::get_peers::GET_PEERS_METHOD;
    pub use crate::rpcs::v1_5_0::get_peers::{GetPeersResult, PeerEntry};
}

pub(crate) mod get_state_root_hash {
    pub use crate::rpcs::v1_5_0::get_state_root_hash::GetStateRootHashResult;
    pub(crate) use crate::rpcs::v1_5_0::get_state_root_hash::{
        GetStateRootHashParams, GET_STATE_ROOT_HASH_METHOD,
    };
}

pub(crate) mod get_validator_changes {
    pub(crate) use crate::rpcs::v1_5_0::get_validator_changes::GET_VALIDATOR_CHANGES_METHOD;
    pub use crate::rpcs::v1_5_0::get_validator_changes::{
        GetValidatorChangesResult, ValidatorChange, ValidatorChangeInEra, ValidatorChanges,
    };
}

pub(crate) mod list_rpcs {
    pub(crate) use crate::rpcs::v1_5_0::list_rpcs::LIST_RPCS_METHOD;
    pub use crate::rpcs::v1_5_0::list_rpcs::{
        Components, Example, ExampleParam, ExampleResult, ListRpcsResult, Method,
        OpenRpcContactField, OpenRpcInfoField, OpenRpcLicenseField, OpenRpcSchema,
        OpenRpcServerEntry, ResponseResult, SchemaParam,
    };
}

pub(crate) mod put_deploy {
    pub use crate::rpcs::v1_5_0::put_deploy::PutDeployResult;
    pub(crate) use crate::rpcs::v1_5_0::put_deploy::{PutDeployParams, PUT_DEPLOY_METHOD};
}
