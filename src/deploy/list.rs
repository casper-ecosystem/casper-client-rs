use std::str;

use async_trait::async_trait;
use clap::{ArgMatches, Command};
use serde::{Deserialize, Serialize};

use casper_client::cli::CliError;
use casper_types::{Block, DeployHash, ProtocolVersion, TransactionHash};

use crate::{command::ClientCommand, common, Success};
use casper_client::rpcs::results::GetBlockResult;

/// This struct defines the order in which the args are shown for this subcommand.
enum DisplayOrder {
    Verbose,
    NodeAddress,
    RpcId,
    BlockHash,
}

/// Result for list-deploys subcommand.
#[derive(Serialize, Deserialize, Debug)]
pub struct ListDeploysResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// The deploy hashes of the block, if found.
    pub deploy_hashes: Option<Vec<DeployHash>>,
    /// The transfer deploy hashes of the block, if found.
    pub transfer_hashes: Option<Vec<DeployHash>>,
}

impl From<GetBlockResult> for ListDeploysResult {
    fn from(get_block_result: GetBlockResult) -> Self {
        ListDeploysResult {
            api_version: get_block_result.api_version,
            deploy_hashes: get_block_result.block_with_signatures.as_ref().map(
                |block| match &block.block {
                    Block::V1(v1_block) => v1_block.deploy_hashes().to_vec(),
                    Block::V2(v2_block) => v2_block
                        .standard()
                        .filter_map(|txn_hash| match txn_hash {
                            TransactionHash::Deploy(deploy_hash) => Some(*deploy_hash),
                            TransactionHash::V1(_) => None,
                        })
                        .collect(),
                },
            ),
            transfer_hashes: get_block_result.block_with_signatures.map(|block| {
                match &block.block {
                    Block::V1(v1_block) => v1_block.transfer_hashes().to_vec(),
                    Block::V2(v2_block) => v2_block
                        .mint()
                        .filter_map(|txn_hash| match txn_hash {
                            TransactionHash::Deploy(deploy_hash) => Some(*deploy_hash),
                            TransactionHash::V1(_) => None,
                        })
                        .collect(),
                }
            }),
        }
    }
}

pub struct ListDeploys;

#[async_trait]
impl ClientCommand for ListDeploys {
    const NAME: &'static str = "list-deploys";
    const ABOUT: &'static str = "Retrieve the list of all deploy hashes in a given block";

    fn build(display_order: usize) -> Command {
        Command::new(Self::NAME)
            .about(Self::ABOUT)
            .display_order(display_order)
            .arg(common::verbose::arg(DisplayOrder::Verbose as usize))
            .arg(common::node_address::arg(
                DisplayOrder::NodeAddress as usize,
            ))
            .arg(common::rpc_id::arg(DisplayOrder::RpcId as usize))
            .arg(common::block_identifier::arg(
                DisplayOrder::BlockHash as usize,
                true,
            ))
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        let maybe_rpc_id = common::rpc_id::get(matches);
        let node_address = common::node_address::get(matches);
        let verbosity_level = common::verbose::get(matches);
        let maybe_block_id = common::block_identifier::get(matches);

        let result = casper_client::cli::get_block(
            maybe_rpc_id,
            node_address,
            verbosity_level,
            maybe_block_id,
        )
        .await;

        result.map(|response| {
            let list = ListDeploysResult::from(response.result);
            Success::Output(serde_json::to_string_pretty(&list).expect("should encode"))
        })
    }
}
