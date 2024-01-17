use casper_types::account::AccountHash;
use casper_types::bytesrepr::Bytes;
use casper_types::{EntityAddr, PackageAddr, PublicKey, URef, U512};

/// An enum representing the parameters needed to construct a transaction builder
/// for the commands concerning the creation of a transaction

pub enum TransactionBuilderParams<'a> {
    /// Parameters for the add bid variant of the transaction builder
    AddBid {
        /// The public key for the add bid transaction
        public_key: PublicKey,
        /// The delegation rate for the add bid transaction
        delegation_rate: u8,
        /// The amount to be bid in the add bid transaction
        amount: U512,
    },
    /// Parameters for the delegate variant of the transaction builder
    Delegate {
        /// The delegator for the delegate transaction
        delegator: PublicKey,
        /// The validator on which to delegate via the transaction
        validator: PublicKey,
        /// The amount to be delegtaed in the transaction
        amount: U512,
    },
    /// Parameters for the undelegate variant of the transaction builder
    Undelegate {
        /// The delegator for the undelegate transaction
        delegator: PublicKey,
        /// The delegator for the delegate transaction
        validator: PublicKey,
        /// The delegator for the delegate transaction
        amount: U512,
    },
    /// Parameters for the redelegate variant of the transaction builder
    Redelegate {
        /// The delegator for the redelegate transaction
        delegator: PublicKey,
        /// The validator for the redelegate transaction
        validator: PublicKey,
        /// The amount to be redelegated for the redelegate transaction
        amount: U512,
        /// The new validator for the redelegate transaction
        new_validator: PublicKey,
    },
    /// Parameters for the invocable entity variant of the transaction builder
    InvocableEntity {
        /// The entity address for the invocable entity transaction
        entity_addr: EntityAddr,
        /// The entry point for the invocable entity transaction
        entry_point: &'a str,
    },
    /// Parameters for the invocable entity alias variant of the transaction builder
    InvocableEntityAlias {
        /// The entity alias for the invocable entity alias transaction
        entity_alias: &'a str,
        /// The entry_point for the invocable entity alias transaction
        entry_point: &'a str,
    },
    /// Parameters for the package variant of the transaction builder
    Package {
        /// The package address for the package transaction
        package_addr: PackageAddr,
        /// The optional entity version for the package transaction
        maybe_entity_version: Option<u32>,
        /// The entry_point for the package transaction
        entry_point: &'a str,
    },
    /// Parameters for the package alias variant of the transaction builder
    PackageAlias {
        /// The package alias for the package alias transaction
        package_alias: &'a str,
        /// The optional entity version for the package alias transaction
        maybe_entity_version: Option<u32>,
        /// The entry point for the package alias transaction
        entry_point: &'a str,
    },
    /// Parameters for the session variant of the transaction builder
    Session {
        /// The Bytes to be run by the execution engine for the session transaction
        transaction_bytes: Bytes,
        /// The entry point for the session transaction
        entry_point: &'a str,
    },
    /// Parameters for the transfer variant of the transaction builder
    Transfer {
        /// The uref for the source purse for the undelegate transaction
        source_uref: URef,
        /// The uref for the traget_purse for the undelegate transaction
        target_uref: URef,
        /// The amount of motes for the undelegate transaction
        amount: U512,
        /// The optional account hash for the transfer transaction
        maybe_to: Option<AccountHash>,
        /// The optional id for the transfer transaction
        maybe_id: Option<u64>,
    },
    /// Parameters for the withdraw bid variant of the transaction builder
    WithdrawBid {
        /// The public key for the withdraw bid transaction
        public_key: PublicKey,
        /// The amount to be withdrawn in the withdraw bid transaction
        amount: U512,
    },
}
