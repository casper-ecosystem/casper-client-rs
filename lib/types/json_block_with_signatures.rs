use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_map_to_array::{BTreeMapToArray, KeyValueLabels};

use casper_types::{Block, PublicKey, Signature};

/// A JSON-friendly representation of a block and the signatures for that block.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct JsonBlockWithSignatures {
    /// The block.
    pub block: Block,
    /// The proofs of the block, i.e. a collection of validators' signatures of the block hash.
    #[serde(with = "BTreeMapToArray::<PublicKey, Signature, BlockProofLabels>")]
    pub proofs: BTreeMap<PublicKey, Signature>,
}

struct BlockProofLabels;

impl KeyValueLabels for BlockProofLabels {
    const KEY: &'static str = "public_key";
    const VALUE: &'static str = "signature";
}
