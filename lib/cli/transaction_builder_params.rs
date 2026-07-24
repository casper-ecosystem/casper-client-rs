use casper_types::contracts::ProtocolVersionMajor;
use casper_types::{
    bytesrepr::Bytes,
    system::auction::{DelegatorKind, Reservation},
    AddressableEntityHash, EntityVersion, PackageHash, PublicKey, TransactionRuntimeParams,
    TransferTarget, URef, U512,
};

/// An enum representing the parameters needed to construct a transaction builder
/// for the commands concerning the creation of a transaction

#[derive(Debug)]
pub enum TransactionBuilderParams<'a> {
    /// Parameters for the add bid variant of the transaction builder
    AddBid {
        /// The public key for the add bid transaction
        public_key: PublicKey,
        /// The delegation rate for the add bid transaction
        delegation_rate: u8,
        /// The amount to be bid in the add bid transaction
        amount: U512,
        /// The minimum amount to be delegated
        minimum_delegation_amount: Option<u64>,
        /// The maximum amount to be delegated
        maximum_delegation_amount: Option<u64>,
        /// Number of delegator slots which can be reserved for specific delegators
        reserved_slots: Option<u32>,
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
    /// Parameters for the change bid public key variant of the transaction builder
    ChangeBidPublicKey {
        /// The validator for the change bid public key transaction
        public_key: PublicKey,
        /// New validator for the change bid public key transaction
        new_public_key: PublicKey,
    },
    /// Parameters for the add reservations variant of the transaction builder
    AddReservations {
        /// List of reservations for the add reservations transaction
        reservations: Vec<Reservation>,
    },
    /// Parameters for the cancel reservations variant of the transaction builder
    CancelReservations {
        /// The validator for the cancel reservations transaction
        validator: PublicKey,
        /// List of delegatora for the cancel reservations transaction
        delegators: Vec<DelegatorKind>,
    },
    /// Parameters for the invocable entity variant of the transaction builder
    InvocableEntity {
        /// The entity hash for the invocable entity transaction
        entity_hash: AddressableEntityHash,
        /// The entry point for the invocable entity transaction
        entry_point: &'a str,
        /// Transaction Runtime params.
        runtime: TransactionRuntimeParams,
    },
    /// Parameters for the invocable entity alias variant of the transaction builder
    InvocableEntityAlias {
        /// The entity alias for the invocable entity alias transaction
        entity_alias: &'a str,
        /// The entry_point for the invocable entity alias transaction
        entry_point: &'a str,
        /// Transaction Runtime params.
        runtime: TransactionRuntimeParams,
    },
    /// Parameters for the package variant of the transaction builder
    Package {
        /// The package hash for the package transaction
        package_hash: PackageHash,
        /// The optional entity version for the package transaction
        maybe_entity_version: Option<u32>,
        /// The entry_point for the package transaction
        entry_point: &'a str,
        /// Transaction Runtime.
        runtime: TransactionRuntimeParams,
    },
    /// Parameters for the package variant of the transaction builder
    PackageWithMajorVersion {
        /// The package hash for the package transaction
        package_hash: PackageHash,
        /// The optional entity version for the package alias transaction
        maybe_entity_version: Option<EntityVersion>,
        /// The entry point for the package alias transaction
        entry_point: &'a str,
        /// Transaction Runtime params.
        runtime: TransactionRuntimeParams,
        /// The protocol version major.
        major_protocol_version: Option<ProtocolVersionMajor>,
    },
    /// Parameters for the package alias variant of the transaction builder
    PackageAlias {
        /// The package alias for the package alias transaction
        package_alias: &'a str,
        /// The optional entity version for the package alias transaction
        maybe_entity_version: Option<u32>,
        /// The entry point for the package alias transaction
        entry_point: &'a str,
        /// Transaction Runtime params.
        runtime: TransactionRuntimeParams,
    },
    /// Parameters for the package alias variant of the transaction builder
    PackageAliasWithMajorVersion {
        /// The package alias for the package alias transaction
        package_alias: &'a str,
        /// The optional entity version for the package alias transaction
        maybe_entity_version: Option<EntityVersion>,
        /// The entry point for the package alias transaction
        entry_point: &'a str,
        /// Transaction Runtime params.
        runtime: TransactionRuntimeParams,
        /// The protocol version major.
        major_protocol_version: Option<ProtocolVersionMajor>,
    },
    /// Parameters for the session variant of the transaction builder
    Session {
        /// Flag determining if the Wasm is an install/upgrade.
        is_install_upgrade: bool,
        /// The Bytes to be run by the execution engine for the session transaction
        transaction_bytes: Bytes,
        /// Transaction Runtime.
        runtime: TransactionRuntimeParams,
    },
    /// Parameters for the transfer variant of the transaction builder
    Transfer {
        /// Source of the transfer transaction
        maybe_source: Option<URef>,
        /// Target of the transfer transaction
        target: TransferTarget,
        /// The amount of motes for the undelegate transaction
        amount: U512,
        /// The optional id for the transfer transaction
        maybe_id: Option<u64>,
    },
    /// Parameters for a native transfer to an EVM address
    EvmTransfer {
        /// Source of the transfer transaction
        maybe_source: Option<URef>,
        /// The 20-byte EVM target address
        target: [u8; 20],
        /// The amount of motes for the transfer transaction
        amount: U512,
        /// The optional id for the transfer transaction
        maybe_id: Option<u64>,
    },
    /// Parameters for the withdraw bid variant of the transaction builder
    WithdrawBid {
        /// The public key for the withdraw bid transaction
        public_key: PublicKey,
        /// The amount to be withdrawn in the withdraw bid transaction
        amount: U512,
        /// Override the min bid amount check
        min_bid_override: bool,
    },
    /// Parameters for the activate bid variant of the transaction builder
    ActivateBid {
        /// The public key for the activate bid transaction
        validator: PublicKey,
    },
}
