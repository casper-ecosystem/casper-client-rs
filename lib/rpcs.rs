//! The JSON-RPC request and response types.

pub mod common;
pub mod results;
/// RPCs provided by the v1.4.5 node.
pub(crate) mod v1_4_5;
/// RPCs provided by the v1.5.0 node.
pub(crate) mod v1_5_0;
/// RPCs provided by the v1.6.0 node.
pub(crate) mod v1_6_0;
/// RPCs provided by the v2.0.0 node.
pub(crate) mod v2_0_0;

pub use v2_0_0::{
    get_account::AccountIdentifier, get_dictionary_item::DictionaryItemIdentifier,
    get_entity::EntityIdentifier, query_balance::PurseIdentifier,
    query_global_state::GlobalStateIdentifier,
};
