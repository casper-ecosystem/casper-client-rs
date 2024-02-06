# Changelog

All notable changes to this project will be documented in this file.  The format is based on [Keep a Changelog].

[comment]: <> (Added:      new features)
[comment]: <> (Changed:    changes in existing functionality)
[comment]: <> (Deprecated: soon-to-be removed features)
[comment]: <> (Removed:    now removed features)
[comment]: <> (Fixed:      any bug fixes)
[comment]: <> (Security:   in case of vulnerabilities)



## Unreleased

### Added
* Add module to support node 2.0.0 RPCs.
* Add `make-transaction` command for creating transactions to support node 2.0.0.
* Add `sign-transaction` command for signing transactions to support node 2.0.0.

### Changed
* Update to match change to node RPC `info_get_deploy`.

### Removed
* Remove following public types which are now available in `casper_types`:
  * `Account` and its related types `ActionThresholds` and `AssociatedKey`
  * `Bid`
  * `BidderAndBid`
  * `Block` and its related types `BlockBody`, `BlockHash`, `BlockHashAndHeight` and `BlockHeader`
  * `ChainspecRawBytes`
  * `Contract`
  * `ContractPackage` and its related types `ContractPackageStatus`, `ContractVersion`, `DisabledVersion` and `Group`
  * `Delegator`
  * `Deploy` and its related types `Approval`, `DeployBuilder`, `DeployHash` and `DeployHeader`
  * `EraEnd` and its related types `EraReport`, `Reward` and `ValidatorWeight`
  * `ExecutableDeployItem`
  * `NamedKey`
  * `Proof`
  * `StoredValue`
  * `TimeDiff`
  * `Timestamp`
  * `TransferTarget`



## Release immediately following 2.0.0

### Added
* Add support for crafting unsigned deploys and transfers by providing an account, but not seccret key, to the `make-deploy` and `make-transfer` subcommands.
* Add an optional flag to retrieve finalized approvals for `info_get_deploy`
* Add support for providing an account identifier (public key, or account hash) for the `state_get_account_info` RPC method.

### Fixed
* Fix `GetBlockResult` to match the node's RPC response via `JsonBlockWithSignatures`, and provide block proofs.


## [2.0.0] - 2023-06-28

### Added
* Add new general-purpose API to library, not specific to CLI consumers.
* Add types to avoid depending upon `casper-node` and `casper-execution-engine` crates.
* Add support for new node RPC method `info_get_chainspec`, used in the binary's new `get-chainspec` subcommand.
* Add support for new node RPC method `info_get_status`, used in the binary's new `get-node-status` subcommand.
* Add support for new node RPC method `info_get_peers`, used in the binary's new `get-peers` subcommand.
* Add support for new node RPC method `query_balance`, used in the binary's new `query-balance` subcommand.
* Add support for new node RPC method `speculative_exec`, by adding a flag in the Deploy related subcommands.
* Add support for passing variable-length byte lists as simple args in payment and session args.
* Add support for passing fixed-length byte arrays as simple args in payment and session args.
* Add support for passing payment and session args as JSON.
* Add support for new `lock_status` field in the the `ContractPackage` value.

### Changed
* Update dependencies.
* Move the previous top-level library API, which targets CLI consumers, to a new module `cli`.
* Rename subcommand `get-account-info` to `get-account` while retaining the previous name as an alias for backwards compatibility.
* Rename subcommand `get-era-info-by-switch-block` to `get-era-info` while retaining the previous name as an alias for backwards compatibility.
* Deprecated `get-balance` subcommand in favor of the newly added `query-balance` subcommand.

### Removed
* Remove the C library support.
* Remove dependency on `casper-node` and `casper-execution-engine` crates.
* Remove support for inputting non-default values for `gas_price` and `dependencies` in `Deploy` creation.
* Remove partial merkle-proof validation of some JSON-RPC responses.

### Fixed
* Restore the `-p` short form of `--public-key` arg for `get-account` and `account-address` subcommands.



## [1.6.0]

### Add
* Added a new subcommand `get-era-summary` which optionally takes a block identifier and returns an era summary from a Casper network.

### Changed
* Update dependencies.



## [1.5.1] - 2023-03-08

### Changed
* Update dependencies.



## [1.5.0] - 2022-05-13

### Changed
* Update dependencies.



## [1.4.4] - 2022-04-06

### Changed
* Update dependencies.



## [1.4.3] - 2021-12-06

### Changed
* Update dependencies, in particular `casper-types` to use fixed checksummed-hex format.



## [1.4.2] - 2021-11-13

### Changed
* Support checksummed-hex encoding of various types like `PublicKey` and `AccountHash`.



## [1.4.1] - 2021-10-23

No changes.



## [1.4.0] - 2021-10-21 [YANKED]

### Added
* RPM package build and publish.
* New client binary command `get-validator-changes` that returns status changes of active validators.
* Add `keygen::generate_files` to FFI.

### Changed
* Support building and testing using stable Rust.
* Support `URef`, `PublicKey` and `AccountHash` as transfer targets for `transfer` and `make-transfer`.

### Fixed
* Stop silently ignoring parse errors for `--session-args-complex` or `--payment-args-complex`.



## [1.3.4] - 2021-10-14

No changes.



## [1.3.3] - 2021-10-14

No changes.



## [1.3.2] - 2021-08-02

No changes.



## [1.3.1] - 2021-07-25

No changes.



## [1.3.0] - 2021-07-20

### Added
* Add support for retrieving historical auction information via the addition of an optional `--block-identifier` arg in the `get-auction-info` subcommand.

### Changed
* Change `account-address` subcommand to output properly formatted string.
* Change `put-deploy` and `make-deploy` subcommands to support transfers.
* Change `make-deploy`, `make-transfer` and `sign-deploy` to not overwrite files unless `--force` is passed.
* Change `make-deploy`, `make-transfer` and `sign-deploy` to use transactional file writing for enhanced safety and reliability.
* Update pinned version of Rust to `nightly-2021-06-17`
* Change the Rust interface of the client library to expose `async` functions, instead of running an executor internally.



## [1.2.1] - 2021-07-17

### Changed
* Minor cleanup.



## [1.2.0] - 2021-05-27

### Added
* Support multisig transfers via new `make-transfer` subcommand.

### Changed
* Change to Apache 2.0 license.
* Make `--transfer-id` a required argument of the relevant subcommands.
* Reduce deploy default time-to-live to 30 minutes.



## [1.1.1] - 2021-04-19

No changes.



## [1.1.0] - 2021-04-13 [YANKED]

No changes.



## [1.0.1] - 2021-04-08

### Changed
* Fail if creating a deploy greater than 1 MiB.



## [1.0.0] - 2021-03-30

### Added
* Initial release of client compatible with Casper mainnet.



[Keep a Changelog]: https://keepachangelog.com/en/1.0.0
[unreleased]: https://github.com/casper-ecosystem/casper-client-rs/compare/v2.0.0...main
[2.0.0]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.6.0...v2.0.0
[1.6.0]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.5.1...v1.6.0
[1.5.1]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.5.0...v1.5.1
[1.5.0]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.4.4...v1.5.0
[1.4.4]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.4.3...v1.4.4
[1.4.3]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.4.2...v1.4.3
[1.4.2]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.4.1...v1.4.2
[1.4.1]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.4.0...v1.4.1
[1.4.0]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.3.4...v1.4.0
[1.3.4]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.3.3...v1.3.4
[1.3.3]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.3.2...v1.3.3
[1.3.2]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.3.1...v1.3.2
[1.3.1]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.3.0...v1.3.1
[1.3.0]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.2.1...v1.3.0
[1.2.1]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.2.0...v1.2.1
[1.2.0]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.1.1...v1.2.0
[1.1.1]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.1.0...v1.1.1
[1.1.0]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.0.1...v1.1.0
[1.0.1]: https://github.com/casper-ecosystem/casper-client-rs/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/casper-ecosystem/casper-client-rs/tree/v1.0.0
