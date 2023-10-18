//! Various data types of the Casper network.

mod auction_state;
mod deploy_execution_info;
mod legacy_execution_result;
mod json_block_with_signatures;


pub use legacy_execution_result::LegacyExecutionResult;
pub use auction_state::AuctionState;
pub use deploy_execution_info::DeployExecutionInfo;
pub use json_block_with_signatures::JsonBlockWithSignatures;
