use casper_types::account::AccountHash;
use casper_types::bytesrepr::Bytes;
use casper_types::{EntityAddr, PackageAddr, PublicKey, URef, U512};

/// An enum representing the parameters needed to construct a transaction builder
/// for the commands concerning the creation of a transaction

pub enum TransactionBuilderParams<'a> {
    AddBid {
        public_key: PublicKey,
        delegation_rate: u8,
        amount: U512,
    },
    Delegate {
        delegator: PublicKey,
        validator: PublicKey,
        amount: U512,
    },
    Undelegate {
        delegator: PublicKey,
        validator: PublicKey,
        amount: U512,
    },
    Redelegate {
        delegator: PublicKey,
        validator: PublicKey,
        amount: U512,
        new_validator: PublicKey,
    },
    InvocableEntity {
        entity_addr: EntityAddr,
        entry_point: &'a str,
    },
    InvocableEntityAlias {
        entity_alias: &'a str,
        entry_point: &'a str,
    },
    Package {
        package_addr: PackageAddr,
        maybe_entity_version: Option<u32>,
        entry_point: &'a str,
    },
    PackageAlias {
        package_alias: &'a str,
        maybe_entity_version: Option<u32>,
        entry_point: &'a str,
    },
    Session {
        transaction_bytes: Bytes,
        entry_point: &'a str,
    },
    Transfer {
        source_uref: URef,
        target_uref: URef,
        amount: U512,
        maybe_to: Option<AccountHash>,
        maybe_id: Option<u64>,
    },
    WithdrawBid {
        public_key: PublicKey,
        amount: U512,
    },
}
