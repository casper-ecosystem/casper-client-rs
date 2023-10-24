//! Various data types of the Casper network.

mod auction_state;
mod deploy_execution_info;
mod json_block_with_signatures;
mod legacy_execution_result;

pub use auction_state::AuctionState;
pub use deploy_execution_info::DeployExecutionInfo;
pub use json_block_with_signatures::JsonBlockWithSignatures;
pub use legacy_execution_result::LegacyExecutionResult;
