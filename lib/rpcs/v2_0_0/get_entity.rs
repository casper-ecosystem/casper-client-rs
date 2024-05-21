use casper_types::{
    account::{Account, AccountHash},
    addressable_entity::NamedKeys,
    AddressableEntity as CasperTypesAddressableEntity, EntityAddr, EntryPointValue,
    ProtocolVersion, PublicKey,
};
use serde::{Deserialize, Serialize};

use crate::rpcs::common::BlockIdentifier;

pub(crate) const GET_ENTITY_METHOD: &str = "state_get_entity";

/// Identifier of an addressable entity.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub enum EntityIdentifier {
    /// The public key of an account.
    PublicKey(PublicKey),
    /// The account hash of an account.
    AccountHash(AccountHash),
    /// The address of an addressable entity.
    EntityAddr(EntityAddr),
}

/// An addressable entity with named keys and entry points.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct AddressableEntity {
    pub entity: CasperTypesAddressableEntity,
    pub named_keys: NamedKeys,
    pub entry_points: Vec<EntryPointValue>,
}

/// An addressable entity or a legacy account.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityOrAccount {
    /// An addressable entity with named keys and entry points.
    AddressableEntity(AddressableEntity),
    /// A legacy account.
    LegacyAccount(Account),
}

impl EntityOrAccount {
    /// Returns the addressable entity if present.
    pub fn addressable_entity(&self) -> Option<AddressableEntity> {
        if let EntityOrAccount::AddressableEntity(addressable_entity) = &self {
            Some(addressable_entity.clone())
        } else {
            None
        }
    }

    /// Returns the legacy account if present.
    pub fn legacy_account(&self) -> Option<Account> {
        if let EntityOrAccount::LegacyAccount(account) = &self {
            Some(account.clone())
        } else {
            None
        }
    }
}

/// Params for "state_get_entity" RPC request
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct GetAddressableEntityParams {
    /// The identifier of the entity.
    entity_identifier: EntityIdentifier,
    /// The block identifier.
    block_identifier: Option<BlockIdentifier>,
}

impl GetAddressableEntityParams {
    /// Returns a new `GetAddressableEntityParams`.
    pub fn new(
        entity_identifier: EntityIdentifier,
        block_identifier: Option<BlockIdentifier>,
    ) -> Self {
        GetAddressableEntityParams {
            entity_identifier,
            block_identifier,
        }
    }
}

/// Result for "state_get_entity" RPC response.
#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetAddressableEntityResult {
    /// The RPC API version.
    pub api_version: ProtocolVersion,
    /// The addressable entity or a legacy account.
    pub entity_result: EntityOrAccount,
    /// The Merkle proof.
    pub merkle_proof: String,
}
