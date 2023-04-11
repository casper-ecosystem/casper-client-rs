use std::str;

use async_trait::async_trait;
use clap::{App, ArgMatches, SubCommand};
use serde::{Serialize, Deserialize};

use casper_client::{Error, GetEraSummary};
use casper_types::ProtocolVersion;

use crate::{command::ClientCommand, common, Success};




enum DisplayOrder {
    Verbose,
    NodeAddress,
    RpcId,
    BlockIdentifier
}

#[async_trait]
impl<'a, 'b> ClientCommand<'a, 'b> for GetEraSummary {
    const NAME: &'static str = "get-era-summary";
    const ABOUT: &'static str = "Retrieves era information from the network";

    fn build(display_order: usize) -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about(Self::ABOUT)
            .display_order(display_order)
            .arg(common::verbose::arg(DisplayOrder::Verbose as usize))
            .arg(common::node_address::arg(
                DisplayOrder::NodeAddress as usize,
            ))
            .arg(common::rpc_id::arg(DisplayOrder::RpcId as usize))
            .arg(common::block_identifier::arg(
                DisplayOrder::BlockIdentifier as usize,
            ))
    }

    async fn run(matches: &ArgMatches<'a>) -> Result<Success, Error> {
        let maybe_rpc_id = common::rpc_id::get(matches);
        let node_address = common::node_address::get(matches);
        let verbosity_level = common::verbose::get(matches);
        let maybe_block_id = common::block_identifier::get(matches);

        casper_client::get_era_summary(
            maybe_rpc_id,
            node_address,
            verbosity_level,
            maybe_block_id,
        )
            .await
            .map(Success::from)
    }




}









