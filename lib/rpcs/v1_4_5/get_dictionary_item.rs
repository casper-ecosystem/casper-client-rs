use serde::{Deserialize, Serialize};

#[cfg(doc)]
use casper_types::{account::Account, Contract};
use casper_types::{Digest, ProtocolVersion, StoredValue, URef};

pub(crate) const GET_DICTIONARY_ITEM_METHOD: &str = "state_get_dictionary_item";

/// The identifier for a dictionary item.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum DictionaryItemIdentifier {
    /// A dictionary item identified via an [`Account`]'s named keys.
    AccountNamedKey {
        /// The [`Key::Account`] as a formatted string, identifying the account whose named keys
        /// contains `dictionary_name`.
        key: String,
        /// The named key under which the dictionary seed `URef` is stored.
        dictionary_name: String,
        /// The key within the dictionary under which the item is held.
        dictionary_item_key: String,
    },
    /// A dictionary item identified via a [`Contract`]'s named keys.
    ContractNamedKey {
        /// The [`Key::Hash`] as a formatted string, identifying the contract whose named keys
        /// contains `dictionary_name`.
        key: String,
        /// The named key under which the dictionary seed `URef` is stored.
        dictionary_name: String,
        /// The key within the dictionary under which the item is held.
        dictionary_item_key: String,
    },
    /// A dictionary item identified via its seed [`URef`].
    URef {
        /// The dictionary's seed `URef`.
        seed_uref: URef,
        /// The key within the dictionary under which the item is held.
        dictionary_item_key: String,
    },
    /// A dictionary item identified via its unique address derived from the dictionary's seed
    /// `URef` and the item's key within the dictionary.  The key must be a `Key::Dictionary`
    /// variant, as a formatted string.
    Dictionary(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct GetDictionaryItemParams {
    state_root_hash: Digest,
    dictionary_identifier: DictionaryItemIdentifier,
}

/// The `result` field of a successful JSON-RPC response to a `state_get_dictionary_item` request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetDictionaryItemResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// The dictionary key under which the value is stored.
    pub dictionary_key: String,
    /// The stored value.
    pub stored_value: StoredValue,
    /// The merkle proof of the value.
    pub merkle_proof: String,
}
