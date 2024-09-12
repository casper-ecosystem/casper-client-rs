use casper_types::{
    AsymmetricType, CLValue, DeployExcessiveSizeError, EntityAddr, ExecutableDeployItem, PublicKey,
    SecretKey, U512,
};

use crate::cli::transaction::create_transaction;
use crate::{Error, OutputKind, MAX_SERIALIZED_SIZE_OF_DEPLOY};

use super::*;

const SAMPLE_ACCOUNT: &str = "01722e1b3d31bef0ba832121bd2941aae6a246d0d05ac95aa16dd587cc5469871d";
const PKG_HASH: &str = "09dcee4b212cfd53642ab323fbef07dafafc6f945a80a00147f62910a915c4e6";
const ENTRYPOINT: &str = "entrypoint";
const VERSION: &str = "2";
const SAMPLE_DEPLOY: &str = r#"{
  "hash": "1053f767f1734e3b5b31253ea680778ac53f134f7c24518bf2c4cbb204852617",
  "header": {
    "account": "01f60bce2bb1059c41910eac1e7ee6c3ef4c8fcc63a901eb9603c1524cadfb0c18",
    "timestamp": "2022-12-11T18:37:06.901Z",
    "ttl": "10s",
    "gas_price": 1,
    "body_hash": "0a80edb81389ead7fb3d6a783355d821313c8baa68718fa7478aa0ca6a6b3b59",
    "dependencies": [],
    "chain_name": "casper-test-chain-name-1"
  },
  "payment": {
    "StoredVersionedContractByHash": {
      "hash": "09dcee4b212cfd53642ab323fbef07dafafc6f945a80a00147f62910a915c4e6",
      "version": 2,
      "entry_point": "entrypoint",
      "args": [
        [
          "name_01",
          {
            "cl_type": "Bool",
            "bytes": "00",
            "parsed": false
          }
        ],
	[
          "name_02",
          {
            "cl_type": "I32",
            "bytes": "2a000000",
            "parsed": 42
          }
        ]
      ]
    }
  },
  "session": {
    "StoredVersionedContractByHash": {
      "hash": "09dcee4b212cfd53642ab323fbef07dafafc6f945a80a00147f62910a915c4e6",
      "version": 2,
      "entry_point": "entrypoint",
      "args": [
        [
          "name_01",
          {
            "cl_type": "Bool",
            "bytes": "00",
            "parsed": false
          }
        ],
        [
          "name_02",
          {
            "cl_type": "I32",
            "bytes": "2a000000",
            "parsed": 42
          }
        ]
      ]
 }
  },
  "approvals": [
    {
      "signer": "01f60bce2bb1059c41910eac1e7ee6c3ef4c8fcc63a901eb9603c1524cadfb0c18",
      "signature": "01d701c27d7dc36b48fa457e4c7cc9999b444d7efb4a118c805b82d1f1af337437d00f9a9562694a7dd707abc01fa0158428a365a970853327d70d6d8f15aeea00"
    },
    {
      "signer": "016e3725ffd940bddb56e692e6309c6c82d2def515421219ddfd1ea0952e52491a",
      "signature": "010a973a45b72208b18da27b25ea62c6be31cd1b53b723b74cdd7e9f356d83df821b6431c973e2f6e24d10fdb213dc5e02d552ba113254e610992b6942ff76390e"
    }
  ]
}"#;

pub fn deploy_params_without_account() -> DeployStrParams<'static> {
    DeployStrParams {
        secret_key: "",
        ttl: "10s",
        chain_name: "casper-test-chain-name-1",
        ..Default::default()
    }
}

pub fn deploy_params_without_secret_key() -> DeployStrParams<'static> {
    DeployStrParams {
        ttl: "10s",
        chain_name: "casper-test-chain-name-1",
        session_account: SAMPLE_ACCOUNT,
        ..Default::default()
    }
}

pub fn deploy_params() -> DeployStrParams<'static> {
    DeployStrParams {
        secret_key: "resources/test.pem",
        ttl: "10s",
        chain_name: "casper-test-chain-name-1",
        ..Default::default()
    }
}

fn args_simple() -> Vec<&'static str> {
    vec!["name_01:bool='false'", "name_02:i32='42'"]
}

#[test]
fn should_create_deploy() {
    let deploy_params = deploy_params();
    let payment_params =
        PaymentStrParams::with_package_hash(PKG_HASH, VERSION, ENTRYPOINT, args_simple(), "");
    let session_params =
        SessionStrParams::with_package_hash(PKG_HASH, VERSION, ENTRYPOINT, args_simple(), "");

    let mut output = Vec::new();

    let deploy =
        deploy::with_payment_and_session(deploy_params, payment_params, session_params, false)
            .unwrap();
    crate::write_deploy(&deploy, &mut output).unwrap();

    // The test output can be used to generate data for SAMPLE_DEPLOY:
    // let secret_key = SecretKey::generate_ed25519().unwrap();
    // deploy.sign(&secret_key);
    // println!("{}", serde_json::to_string_pretty(&deploy).unwrap());

    let result = String::from_utf8(output).unwrap();

    let expected = crate::read_deploy(SAMPLE_DEPLOY.as_bytes()).unwrap();
    let actual = crate::read_deploy(result.as_bytes()).unwrap();

    assert_eq!(expected.header().account(), actual.header().account());
    assert_eq!(expected.header().ttl(), actual.header().ttl());
    assert_eq!(expected.header().gas_price(), actual.header().gas_price());
    assert_eq!(expected.header().body_hash(), actual.header().body_hash());
    assert_eq!(expected.payment(), actual.payment());
    assert_eq!(expected.session(), actual.session());
}

#[test]
fn should_fail_to_create_large_deploy() {
    let deploy_params = deploy_params();
    let payment_params =
        PaymentStrParams::with_package_hash(PKG_HASH, VERSION, ENTRYPOINT, args_simple(), "");
    // Create a string arg of 1048576 letter 'a's to ensure the deploy is greater than 1048576
    // bytes.
    let large_args_simple = format!("name_01:string='{:a<1048576}'", "");

    let session_params = SessionStrParams::with_package_hash(
        PKG_HASH,
        VERSION,
        ENTRYPOINT,
        vec![large_args_simple.as_str()],
        "",
    );

    match deploy::with_payment_and_session(deploy_params, payment_params, session_params, false) {
        Err(CliError::Core(Error::DeploySize(DeployExcessiveSizeError {
            max_transaction_size,
            actual_deploy_size,
        }))) => {
            assert_eq!(max_transaction_size, MAX_SERIALIZED_SIZE_OF_DEPLOY);
            assert!(actual_deploy_size > MAX_SERIALIZED_SIZE_OF_DEPLOY as usize);
        }
        Err(error) => panic!("unexpected error: {}", error),
        Ok(_) => panic!("failed to error while creating an excessively large deploy"),
    }
}

#[test]
fn should_read_deploy() {
    let bytes = SAMPLE_DEPLOY.as_bytes();
    assert!(crate::read_deploy(bytes).is_ok());
}

#[test]
fn should_sign_deploy() {
    let bytes = SAMPLE_DEPLOY.as_bytes();
    let deploy = crate::read_deploy(bytes).unwrap();
    assert_eq!(
        deploy.approvals().len(),
        2,
        "Sample deploy should have 2 approvals."
    );

    let tempdir = tempfile::tempdir().unwrap();
    let path = tempdir.path().join("deploy.json");

    crate::output_deploy(OutputKind::file(&path, false), &deploy).unwrap();

    let secret_key = SecretKey::generate_ed25519().unwrap();
    crate::sign_deploy_file(&path, &secret_key, OutputKind::file(&path, true)).unwrap();
    let signed_deploy = crate::read_deploy_file(&path).unwrap();

    assert_eq!(
        signed_deploy.approvals().len(),
        deploy.approvals().len() + 1,
    );
}

#[test]
fn should_create_transfer() {
    use casper_types::{AsymmetricType, PublicKey};

    // with public key.
    let secret_key = SecretKey::generate_ed25519().unwrap();
    let public_key = PublicKey::from(&secret_key).to_hex();
    let transfer_deploy = deploy::new_transfer(
        "10000",
        None,
        &public_key,
        "1",
        deploy_params(),
        PaymentStrParams::with_amount("100"),
        false,
    );

    assert!(transfer_deploy.is_ok());
    assert!(matches!(
        transfer_deploy.unwrap().session(),
        ExecutableDeployItem::Transfer { .. }
    ));

    // with account hash
    let account_hash =
        "account-hash-0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20";
    let transfer_deploy = deploy::new_transfer(
        "10000",
        None,
        account_hash,
        "1",
        deploy_params(),
        PaymentStrParams::with_amount("100"),
        false,
    );

    assert!(transfer_deploy.is_ok());
    assert!(matches!(
        transfer_deploy.unwrap().session(),
        ExecutableDeployItem::Transfer { .. }
    ));

    // with uref.
    let uref = "uref-0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20-007";
    let transfer_deploy = deploy::new_transfer(
        "10000",
        None,
        uref,
        "1",
        deploy_params(),
        PaymentStrParams::with_amount("100"),
        false,
    );

    assert!(transfer_deploy.is_ok());
    assert!(matches!(
        transfer_deploy.unwrap().session(),
        ExecutableDeployItem::Transfer { .. }
    ));
}

#[test]
fn should_fail_to_create_transfer_with_bad_args() {
    let transfer_deploy = deploy::new_transfer(
        "10000",
        None,
        "bad public key.",
        "1",
        deploy_params(),
        PaymentStrParams::with_amount("100"),
        false,
    );

    println!("{:?}", transfer_deploy);

    assert!(matches!(
        transfer_deploy,
        Err(CliError::InvalidArgument {
            context: "new_transfer target_account",
            error: _
        })
    ));
}

#[test]
fn should_create_unsigned_deploy() {
    let deploy_params = deploy_params_without_secret_key();
    let payment_params =
        PaymentStrParams::with_package_hash(PKG_HASH, VERSION, ENTRYPOINT, args_simple(), "");
    let session_params =
        SessionStrParams::with_package_hash(PKG_HASH, VERSION, ENTRYPOINT, args_simple(), "");

    let deploy =
        deploy::with_payment_and_session(deploy_params, payment_params, session_params, true)
            .unwrap();

    assert!(deploy.approvals().is_empty());
    assert_eq!(
        *deploy.header().account(),
        PublicKey::from_hex(SAMPLE_ACCOUNT).unwrap()
    );
}

#[test]
fn should_fail_to_create_deploy_with_no_session_account() {
    let deploy_params = deploy_params_without_account();
    let payment_params =
        PaymentStrParams::with_package_hash(PKG_HASH, VERSION, ENTRYPOINT, args_simple(), "");
    let session_params =
        SessionStrParams::with_package_hash(PKG_HASH, VERSION, ENTRYPOINT, args_simple(), "");

    let deploy =
        deploy::with_payment_and_session(deploy_params, payment_params, session_params, true);
    assert!(deploy.is_err());
    assert!(matches!(
        deploy.unwrap_err(),
        CliError::Core(Error::DeployBuild(
            casper_types::DeployBuilderError::DeployMissingSessionAccount
        ))
    ));
}

#[test]
fn should_create_unsigned_transfer() {
    use casper_types::{AsymmetricType, PublicKey};

    // with public key.
    let secret_key = SecretKey::generate_ed25519().unwrap();
    let public_key = PublicKey::from(&secret_key).to_hex();
    let transfer_deploy = deploy::new_transfer(
        "10000",
        None,
        &public_key,
        "1",
        deploy_params_without_secret_key(),
        PaymentStrParams::with_amount("100"),
        true,
    )
    .unwrap();
    assert!(transfer_deploy.approvals().is_empty());
}

#[test]
fn should_fail_to_create_transfer_without_account() {
    use casper_types::{AsymmetricType, PublicKey};

    // with public key.
    let secret_key = SecretKey::generate_ed25519().unwrap();
    let public_key = PublicKey::from(&secret_key).to_hex();

    let transfer_deploy = deploy::new_transfer(
        "10000",
        None,
        &public_key,
        "1",
        deploy_params_without_account(),
        PaymentStrParams::with_amount("100"),
        true,
    );
    assert!(transfer_deploy.is_err());
    assert!(matches!(
        transfer_deploy.unwrap_err(),
        CliError::Core(Error::DeployBuild(
            casper_types::DeployBuilderError::DeployMissingSessionAccount
        ))
    ));
}

#[test]
fn should_fail_to_create_transfer_with_no_secret_key_while_not_allowing_unsigned_deploy() {
    let deploy_params = deploy_params_without_secret_key();
    let payment_params =
        PaymentStrParams::with_package_hash(PKG_HASH, VERSION, ENTRYPOINT, args_simple(), "");

    // with public key.
    let secret_key = SecretKey::generate_ed25519().unwrap();
    let public_key = PublicKey::from(&secret_key).to_hex();

    let transfer_deploy = deploy::new_transfer(
        "10000",
        None,
        &public_key,
        "1",
        deploy_params,
        payment_params,
        false,
    );

    assert!(transfer_deploy.is_err());
    assert!(matches!(
        transfer_deploy.unwrap_err(),
        CliError::InvalidArgument {
            context: "new_transfer (secret_key, allow_unsigned_deploy)",
            error: _
        }
    ));
}

#[test]
fn should_fail_to_create_deploy_with_payment_and_session_with_no_secret_key_while_not_allowing_unsigned_deploy(
) {
    let deploy_params = deploy_params_without_secret_key();
    let payment_params =
        PaymentStrParams::with_package_hash(PKG_HASH, VERSION, ENTRYPOINT, args_simple(), "");
    let session_params =
        SessionStrParams::with_package_hash(PKG_HASH, VERSION, ENTRYPOINT, args_simple(), "");

    let transfer_deploy =
        deploy::with_payment_and_session(deploy_params, payment_params, session_params, false);

    assert!(transfer_deploy.is_err());
    assert!(matches!(
        transfer_deploy.unwrap_err(),
        CliError::InvalidArgument {
            context: "with_payment_and_session (secret_key, allow_unsigned_deploy)",
            error: _
        }
    ));
}

mod transaction {
    use super::*;
    use crate::Error::TransactionBuild;
    use casper_types::{
        bytesrepr::Bytes, PackageAddr, TransactionEntryPoint, TransactionInvocationTarget,
        TransactionRuntime, TransactionTarget, TransactionV1BuilderError, TransferTarget,
    };
    const SAMPLE_TRANSACTION: &str = r#"{
  "serialization_version": 1,
  "hash": "f868596bbfd729547ffa25c3421df29d6650cec73e9fe3d0aff633fe2d6ac952",
  "header": {
    "chain_name": "test",
    "timestamp": "2024-01-26T19:08:53.498Z",
    "ttl": "30m",
    "body_hash": "fb94fd83178e3acf22546beebf5f44692499d681c4381f6d145d85ff9b5fc152",
    "pricing_mode": {
      "Fixed": {
        "gas_price_tolerance": 10
      }
    },
    "initiator_addr": {
      "PublicKey": "01722e1b3d31bef0ba832121bd2941aae6a246d0d05ac95aa16dd587cc5469871d"
    }
  },
  "body": {
    "args": [
      [
        "source",
        {
          "cl_type": "URef",
          "bytes": "722e1b3d31bef0ba832121bd2941aae6a246d0d05ac95aa16dd587cc5469871d01",
          "parsed": "uref-722e1b3d31bef0ba832121bd2941aae6a246d0d05ac95aa16dd587cc5469871d-001"
        }
      ],
      [
        "target",
        {
          "cl_type": "URef",
          "bytes": "722e1b3d31bef0ba832121bd2941aae6a246d0d05ac95aa16dd587cc5469871d01",
          "parsed": "uref-722e1b3d31bef0ba832121bd2941aae6a246d0d05ac95aa16dd587cc5469871d-001"
        }
      ],
      [
        "amount",
        {
          "cl_type": "U512",
          "bytes": "010a",
          "parsed": "10"
        }
      ]
    ],
    "target": "Native",
    "entry_point": "Transfer",
    "transaction_category": 0,
    "scheduling": "Standard"
  },
  "approvals": []
}
"#;
    const SAMPLE_DIGEST: &str =
        "01722e1b3d31bef0ba832121bd2941aae6a246d0d05ac95aa16dd587cc5469871d";

    #[test]
    fn should_sign_transaction() {
        let bytes = SAMPLE_TRANSACTION.as_bytes();
        let transaction = crate::read_transaction(bytes).unwrap();
        assert_eq!(
            transaction.approvals().len(),
            0,
            "Sample transaction should have 0 approvals."
        );

        let tempdir = tempfile::tempdir().unwrap();
        let path = tempdir.path().join("deploy.json");

        crate::output_transaction(OutputKind::file(&path, false), &transaction).unwrap();

        let secret_key = SecretKey::generate_ed25519().unwrap();
        crate::sign_transaction_file(&path, &secret_key, OutputKind::file(&path, true)).unwrap();
        let signed_transaction = crate::read_transaction_file(&path).unwrap();

        assert_eq!(
            signed_transaction.approvals().len(),
            transaction.approvals().len() + 1,
        );
    }

    #[test]
    fn should_create_add_bid_transaction() {
        let secret_key = SecretKey::generate_ed25519().unwrap();
        let amount = U512::from(1000);
        let minimum_delegation_amount = 100u64;
        let maximum_delegation_amount = 10000u64;
        let public_key = PublicKey::from(&secret_key);

        let amount_cl = &CLValue::from_t(amount).unwrap();
        let public_key_cl = &CLValue::from_t(&public_key).unwrap();

        let transaction_string_params = TransactionStrParams {
            secret_key: "",
            timestamp: "",
            ttl: "30min",
            chain_name: "add-bid-test",
            initiator_addr: SAMPLE_ACCOUNT.to_string(),
            session_args_simple: vec![],
            session_args_json: "",
            pricing_mode: "fixed",
            output_path: "",
            payment_amount: "100",
            gas_price_tolerance: "10",
            receipt: SAMPLE_DIGEST,
            standard_payment: "true",
        };

        let transaction_builder_params = TransactionBuilderParams::AddBid {
            public_key,
            delegation_rate: 0,
            amount,
            minimum_delegation_amount,
            maximum_delegation_amount,
        };

        let transaction =
            create_transaction(transaction_builder_params, transaction_string_params, true);

        assert!(transaction.is_ok(), "{:?}", transaction);
        assert_eq!(transaction.as_ref().unwrap().chain_name(), "add-bid-test");
        assert_eq!(
            transaction
                .as_ref()
                .unwrap()
                .args()
                .get("public_key")
                .unwrap(),
            public_key_cl
        );
        assert!(transaction
            .as_ref()
            .unwrap()
            .args()
            .get("delegation_rate")
            .is_some());
        assert_eq!(
            transaction.as_ref().unwrap().args().get("amount").unwrap(),
            amount_cl
        );
    }
    #[test]
    fn should_create_delegate_transaction() {
        let delegator_secret_key = SecretKey::generate_ed25519().unwrap();
        let validator_secret_key = SecretKey::generate_ed25519().unwrap();

        let delegator_public_key = PublicKey::from(&delegator_secret_key);
        let validator_public_key = PublicKey::from(&validator_secret_key);
        let amount = U512::from(2000);

        let delegator_public_key_cl = &CLValue::from_t(delegator_public_key).unwrap();
        let validator_public_key_cl = &CLValue::from_t(validator_public_key).unwrap();
        let amount_cl = &CLValue::from_t(amount).unwrap();

        let transaction_string_params = TransactionStrParams {
            secret_key: "",
            timestamp: "",
            ttl: "30min",
            chain_name: "delegate",
            initiator_addr: SAMPLE_ACCOUNT.to_string(),
            session_args_simple: vec![],
            session_args_json: "",
            pricing_mode: "fixed",
            output_path: "",
            payment_amount: "100",
            gas_price_tolerance: "10",
            receipt: SAMPLE_DIGEST,
            standard_payment: "true",
        };

        let transaction_builder_params = TransactionBuilderParams::Delegate {
            delegator: PublicKey::from(&delegator_secret_key),
            validator: PublicKey::from(&validator_secret_key),
            amount,
        };

        let transaction =
            create_transaction(transaction_builder_params, transaction_string_params, true);

        assert!(transaction.is_ok(), "{:?}", transaction);
        assert_eq!(transaction.as_ref().unwrap().chain_name(), "delegate");
        assert_eq!(
            transaction.as_ref().unwrap().args().get("amount").unwrap(),
            amount_cl
        );
        assert_eq!(
            transaction
                .as_ref()
                .unwrap()
                .args()
                .get("delegator")
                .unwrap(),
            delegator_public_key_cl
        );
        assert_eq!(
            transaction
                .as_ref()
                .unwrap()
                .args()
                .get("validator")
                .unwrap(),
            validator_public_key_cl
        );
    }

    #[test]
    fn should_create_withdraw_bid_transaction() {
        let secret_key = SecretKey::generate_ed25519().unwrap();

        let public_key = PublicKey::from(&secret_key);
        let amount = U512::from(3000);

        let public_key_cl = &CLValue::from_t(&public_key).unwrap();
        let amount_cl = &CLValue::from_t(amount).unwrap();

        let transaction_string_params = TransactionStrParams {
            secret_key: "",
            timestamp: "",
            ttl: "30min",
            chain_name: "withdraw-bid",
            initiator_addr: SAMPLE_ACCOUNT.to_string(),
            session_args_simple: vec![],
            session_args_json: "",
            pricing_mode: "fixed",
            output_path: "",
            payment_amount: "100",
            gas_price_tolerance: "10",
            receipt: SAMPLE_DIGEST,
            standard_payment: "true",
        };

        let transaction_builder_params =
            TransactionBuilderParams::WithdrawBid { public_key, amount };

        let transaction =
            create_transaction(transaction_builder_params, transaction_string_params, true);

        assert!(transaction.is_ok(), "{:?}", transaction);
        assert_eq!(transaction.as_ref().unwrap().chain_name(), "withdraw-bid");
        assert_eq!(
            transaction.as_ref().unwrap().args().get("amount").unwrap(),
            amount_cl
        );
        assert_eq!(
            transaction
                .as_ref()
                .unwrap()
                .args()
                .get("public_key")
                .unwrap(),
            public_key_cl
        );
    }

    #[test]
    fn should_create_undelegatge_transaction() {
        let delegator_secret_key = SecretKey::generate_ed25519().unwrap();
        let validator_secret_key = SecretKey::generate_ed25519().unwrap();

        let amount = U512::from(4000);
        let delegator_public_key = PublicKey::from(&delegator_secret_key);
        let validator_public_key = PublicKey::from(&validator_secret_key);

        let amount_cl = &CLValue::from_t(amount).unwrap();
        let delegator_public_key_cl = &CLValue::from_t(delegator_public_key).unwrap();
        let validator_public_key_cl = &CLValue::from_t(validator_public_key).unwrap();

        let transaction_string_params = TransactionStrParams {
            secret_key: "",
            timestamp: "",
            ttl: "30min",
            chain_name: "undelegate",
            initiator_addr: SAMPLE_ACCOUNT.to_string(),
            session_args_simple: vec![],
            session_args_json: "",
            pricing_mode: "fixed",
            output_path: "",
            payment_amount: "100",
            gas_price_tolerance: "10",
            receipt: SAMPLE_DIGEST,
            standard_payment: "true",
        };

        let transaction_builder_params = TransactionBuilderParams::Undelegate {
            delegator: PublicKey::from(&delegator_secret_key),
            validator: PublicKey::from(&validator_secret_key),
            amount,
        };

        let transaction =
            create_transaction(transaction_builder_params, transaction_string_params, true);

        assert!(transaction.is_ok(), "{:?}", transaction);
        assert_eq!(transaction.as_ref().unwrap().chain_name(), "undelegate");
        assert_eq!(
            transaction.as_ref().unwrap().args().get("amount").unwrap(),
            amount_cl
        );
        assert_eq!(
            transaction
                .as_ref()
                .unwrap()
                .args()
                .get("delegator")
                .unwrap(),
            delegator_public_key_cl
        );
        assert_eq!(
            transaction
                .as_ref()
                .unwrap()
                .args()
                .get("validator")
                .unwrap(),
            validator_public_key_cl
        );
    }

    #[test]
    fn should_create_redelegatge_transaction() {
        let delegator_secret_key = SecretKey::generate_ed25519().unwrap();
        let validator_secret_key = SecretKey::generate_ed25519().unwrap();
        let new_validator_secret_key = SecretKey::generate_ed25519().unwrap();

        let delegator_public_key = PublicKey::from(&delegator_secret_key);
        let validator_public_key = PublicKey::from(&validator_secret_key);
        let new_validator_public_key = PublicKey::from(&new_validator_secret_key);
        let amount = U512::from(5000);

        let delegator_public_key_cl = &CLValue::from_t(delegator_public_key).unwrap();
        let validator_public_key_cl = &CLValue::from_t(validator_public_key).unwrap();
        let new_validator_public_key_cl = &CLValue::from_t(new_validator_public_key).unwrap();
        let amount_cl = &CLValue::from_t(amount).unwrap();

        let transaction_string_params = TransactionStrParams {
            secret_key: "",
            timestamp: "",
            ttl: "30min",
            chain_name: "redelegate",
            initiator_addr: SAMPLE_ACCOUNT.to_string(),
            session_args_simple: vec![],
            session_args_json: "",
            pricing_mode: "fixed",
            output_path: "",
            payment_amount: "100",
            gas_price_tolerance: "10",
            receipt: SAMPLE_DIGEST,
            standard_payment: "true",
        };

        let transaction_builder_params = TransactionBuilderParams::Redelegate {
            delegator: PublicKey::from(&delegator_secret_key),
            validator: PublicKey::from(&validator_secret_key),
            amount,
            new_validator: PublicKey::from(&new_validator_secret_key),
        };
        let transaction =
            create_transaction(transaction_builder_params, transaction_string_params, true);
        assert!(transaction.is_ok(), "{:?}", transaction);
        assert_eq!(transaction.as_ref().unwrap().chain_name(), "redelegate");
        assert_eq!(
            transaction.as_ref().unwrap().args().get("amount").unwrap(),
            amount_cl
        );
        assert_eq!(
            transaction
                .as_ref()
                .unwrap()
                .args()
                .get("delegator")
                .unwrap(),
            delegator_public_key_cl
        );
        assert_eq!(
            transaction
                .as_ref()
                .unwrap()
                .args()
                .get("validator")
                .unwrap(),
            validator_public_key_cl
        );
        assert_eq!(
            transaction
                .as_ref()
                .unwrap()
                .args()
                .get("new_validator")
                .unwrap(),
            new_validator_public_key_cl
        );
    }

    #[test]
    fn should_create_invocable_entity_transaction() {
        let entity_addr: EntityAddr = EntityAddr::new_account([0u8; 32]);
        let entity_hash = entity_addr.value();
        let entry_point = String::from("test-entry-point");
        let target = &TransactionTarget::Stored {
            id: TransactionInvocationTarget::ByHash(entity_hash),
            runtime: TransactionRuntime::VmCasperV1,
        };

        let entry_point_ref = &TransactionEntryPoint::Custom(entry_point);

        let transaction_string_params = TransactionStrParams {
            secret_key: "",
            timestamp: "",
            ttl: "30min",
            chain_name: "invocable-entity",
            initiator_addr: SAMPLE_ACCOUNT.to_string(),
            session_args_simple: vec![],
            session_args_json: "",
            pricing_mode: "fixed",
            output_path: "",
            payment_amount: "100",
            gas_price_tolerance: "10",
            receipt: SAMPLE_DIGEST,
            standard_payment: "true",
        };

        let transaction_builder_params = TransactionBuilderParams::InvocableEntity {
            entity_hash: entity_hash.into(),
            entry_point: "test-entry-point",
        };
        let transaction =
            create_transaction(transaction_builder_params, transaction_string_params, true);

        assert!(transaction.is_ok(), "{:?}", transaction);
        assert_eq!(
            transaction.as_ref().unwrap().chain_name(),
            "invocable-entity"
        );
        assert_eq!(
            transaction.as_ref().unwrap().body().entry_point(),
            entry_point_ref
        );
        assert_eq!(transaction.as_ref().unwrap().body().target(), target);
    }
    #[test]
    fn should_create_invocable_entity_alias_transaction() {
        let alias = String::from("alias");
        let target = &TransactionTarget::Stored {
            id: TransactionInvocationTarget::ByName(alias),
            runtime: TransactionRuntime::VmCasperV1,
        };
        let transaction_string_params = TransactionStrParams {
            secret_key: "",
            timestamp: "",
            ttl: "30min",
            chain_name: "invocable-entity-alias",
            initiator_addr: SAMPLE_ACCOUNT.to_string(),
            session_args_simple: vec![],
            session_args_json: "",
            pricing_mode: "fixed",
            output_path: "",
            payment_amount: "100",
            gas_price_tolerance: "10",
            receipt: SAMPLE_DIGEST,
            standard_payment: "true",
        };

        let transaction_builder_params = TransactionBuilderParams::InvocableEntityAlias {
            entity_alias: "alias",
            entry_point: "entry-point-alias",
        };
        let transaction =
            create_transaction(transaction_builder_params, transaction_string_params, true);
        assert!(transaction.is_ok(), "{:?}", transaction);
        assert_eq!(
            transaction.as_ref().unwrap().chain_name(),
            "invocable-entity-alias"
        );
        assert_eq!(
            transaction.as_ref().unwrap().body().entry_point(),
            &TransactionEntryPoint::Custom("entry-point-alias".to_string())
        );
        assert_eq!(transaction.as_ref().unwrap().body().target(), target);
    }
    #[test]
    fn should_create_package_transaction() {
        let package_addr: PackageAddr = vec![0u8; 32].as_slice().try_into().unwrap();
        let entry_point = "test-entry-point-package";
        let maybe_entity_version = Some(23);
        let target = &TransactionTarget::Stored {
            id: TransactionInvocationTarget::ByPackageHash {
                addr: package_addr,
                version: maybe_entity_version,
            },
            runtime: TransactionRuntime::VmCasperV1,
        };
        let transaction_string_params = TransactionStrParams {
            secret_key: "",
            timestamp: "",
            ttl: "30min",
            chain_name: "package",
            initiator_addr: SAMPLE_ACCOUNT.to_string(),
            session_args_simple: vec![],
            session_args_json: "",
            pricing_mode: "fixed",
            output_path: "",
            payment_amount: "100",
            gas_price_tolerance: "10",
            receipt: SAMPLE_DIGEST,
            standard_payment: "true",
        };

        let transaction_builder_params = TransactionBuilderParams::Package {
            package_hash: package_addr.into(),
            entry_point,
            maybe_entity_version,
        };
        let transaction =
            create_transaction(transaction_builder_params, transaction_string_params, true);
        assert!(transaction.is_ok(), "{:?}", transaction);
        assert_eq!(transaction.as_ref().unwrap().chain_name(), "package");
        assert_eq!(
            transaction.as_ref().unwrap().body().entry_point(),
            &TransactionEntryPoint::Custom("test-entry-point-package".to_string())
        );
        assert_eq!(transaction.as_ref().unwrap().body().target(), target);
    }
    #[test]
    fn should_create_package_alias_transaction() {
        let package_name = String::from("package-name");
        let entry_point = "test-entry-point-package";
        let maybe_entity_version = Some(23);
        let target = &TransactionTarget::Stored {
            id: TransactionInvocationTarget::ByPackageName {
                name: package_name.clone(),
                version: maybe_entity_version,
            },
            runtime: TransactionRuntime::VmCasperV1,
        };
        let transaction_string_params = TransactionStrParams {
            secret_key: "",
            timestamp: "",
            ttl: "30min",
            chain_name: "package",
            initiator_addr: SAMPLE_ACCOUNT.to_string(),
            session_args_simple: vec![],
            session_args_json: "",
            pricing_mode: "fixed",
            output_path: "",
            payment_amount: "100",
            gas_price_tolerance: "10",
            receipt: SAMPLE_DIGEST,
            standard_payment: "true",
        };

        let transaction_builder_params = TransactionBuilderParams::PackageAlias {
            package_alias: &package_name,
            entry_point,
            maybe_entity_version,
        };
        let transaction =
            create_transaction(transaction_builder_params, transaction_string_params, true);
        assert!(transaction.is_ok(), "{:?}", transaction);
        assert_eq!(transaction.as_ref().unwrap().chain_name(), "package");
        assert_eq!(
            transaction.as_ref().unwrap().body().entry_point(),
            &TransactionEntryPoint::Custom("test-entry-point-package".to_string())
        );
        assert_eq!(transaction.as_ref().unwrap().body().target(), target);
    }
    #[test]
    fn should_create_session_transaction() {
        let transaction_bytes = Bytes::from(vec![1u8; 32]);
        let target = &TransactionTarget::Session {
            runtime: TransactionRuntime::VmCasperV1,
            module_bytes: transaction_bytes.clone(),
        };
        let transaction_string_params = TransactionStrParams {
            secret_key: "",
            timestamp: "",
            ttl: "30min",
            chain_name: "session",
            initiator_addr: SAMPLE_ACCOUNT.to_string(),
            session_args_simple: vec![],
            session_args_json: "",
            pricing_mode: "fixed",
            output_path: "",
            payment_amount: "100",
            gas_price_tolerance: "10",
            receipt: SAMPLE_DIGEST,
            standard_payment: "true",
        };

        let transaction_builder_params = TransactionBuilderParams::Session {
            transaction_bytes,
            transaction_category: casper_types::TransactionCategory::Large,
        };
        let transaction =
            create_transaction(transaction_builder_params, transaction_string_params, true);
        assert!(transaction.is_ok(), "{:?}", transaction);
        assert_eq!(transaction.as_ref().unwrap().chain_name(), "session");
        assert_eq!(
            transaction.as_ref().unwrap().body().entry_point(),
            &TransactionEntryPoint::Call
        );
        assert_eq!(transaction.as_ref().unwrap().body().target(), target);
    }
    #[test]
    fn should_create_transfer_transaction() {
        let source_uref = URef::from_formatted_str(
            "uref-0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20-007",
        )
        .unwrap();
        let target_uref = URef::from_formatted_str(
            "uref-0202030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20-007",
        )
        .unwrap();

        let transfer_target = TransferTarget::URef(target_uref);

        let maybe_source = Some(source_uref);

        let source_uref_cl = &CLValue::from_t(Some(&source_uref)).unwrap();
        let target_uref_cl = &CLValue::from_t(target_uref).unwrap();

        let transaction_string_params = TransactionStrParams {
            secret_key: "",
            timestamp: "",
            ttl: "30min",
            chain_name: "transfer",
            initiator_addr: SAMPLE_ACCOUNT.to_string(),
            session_args_simple: vec![],
            session_args_json: "",
            pricing_mode: "fixed",
            output_path: "",
            payment_amount: "100",
            gas_price_tolerance: "10",
            receipt: SAMPLE_DIGEST,
            standard_payment: "true",
        };

        let transaction_builder_params = TransactionBuilderParams::Transfer {
            maybe_source,
            target: transfer_target,
            amount: Default::default(),
            maybe_id: None,
        };
        let transaction =
            create_transaction(transaction_builder_params, transaction_string_params, true);
        assert!(transaction.is_ok(), "{:?}", transaction);
        assert_eq!(transaction.as_ref().unwrap().chain_name(), "transfer");
        assert_eq!(
            transaction.as_ref().unwrap().body().entry_point(),
            &TransactionEntryPoint::Transfer
        );
        assert_eq!(
            transaction.as_ref().unwrap().args().get("source").unwrap(),
            source_uref_cl
        );
        assert_eq!(
            transaction.as_ref().unwrap().args().get("target").unwrap(),
            target_uref_cl
        );
    }
    #[test]
    fn should_fail_to_create_transaction_with_no_secret_or_public_key() {
        let transaction_string_params = TransactionStrParams {
            secret_key: "",
            timestamp: "",
            ttl: "30min",
            chain_name: "no-secret",
            initiator_addr: "".to_string(),
            session_args_simple: vec![],
            session_args_json: "",
            pricing_mode: "fixed",
            output_path: "",
            payment_amount: "100",
            gas_price_tolerance: "10",
            receipt: SAMPLE_DIGEST,
            standard_payment: "true",
        };
        let transaction_builder_params = TransactionBuilderParams::Transfer {
            maybe_source: Default::default(),
            target: TransferTarget::URef(Default::default()),
            amount: Default::default(),
            maybe_id: None,
        };
        let transaction =
            create_transaction(transaction_builder_params, transaction_string_params, true);
        assert!(transaction.is_err());
        assert!(matches!(
            transaction.unwrap_err(),
            CliError::Core(TransactionBuild(
                TransactionV1BuilderError::MissingInitiatorAddr
            ))
        ));
    }
    #[test]
    fn should_create_transaction_with_secret_key_but_no_initiator_addr() {
        let minimum_delegation_amount = 100u64;
        let maximum_delegation_amount = 10000u64;

        let transaction_string_params = TransactionStrParams {
            secret_key: "resources/test.pem",
            timestamp: "",
            ttl: "30min",
            chain_name: "has-secret",
            initiator_addr: "".to_string(),
            session_args_simple: vec![],
            session_args_json: "",
            pricing_mode: "fixed",
            output_path: "",
            payment_amount: "100",
            gas_price_tolerance: "10",
            receipt: SAMPLE_DIGEST,
            standard_payment: "true",
        };
        let transaction_builder_params = TransactionBuilderParams::AddBid {
            public_key: PublicKey::from_hex(SAMPLE_ACCOUNT).unwrap(),
            delegation_rate: 0,
            amount: U512::from(10),
            minimum_delegation_amount,
            maximum_delegation_amount,
        };
        let transaction =
            create_transaction(transaction_builder_params, transaction_string_params, true);
        assert!(transaction.is_ok(), "{:?}", transaction);
        println!("{:?}", transaction);
    }

    #[test]
    fn should_fail_to_create_transaction_with_no_secret_and_no_unsigned_transactions() {
        let minimum_delegation_amount = 100u64;
        let maximum_delegation_amount = 10000u64;

        let transaction_string_params = TransactionStrParams {
            secret_key: "",
            timestamp: "",
            ttl: "30min",
            chain_name: "no-secret-must-be-signed",
            initiator_addr: SAMPLE_ACCOUNT.to_string(),
            session_args_simple: vec![],
            session_args_json: "",
            pricing_mode: "fixed",
            output_path: "",
            payment_amount: "100",
            gas_price_tolerance: "",
            receipt: SAMPLE_DIGEST,
            standard_payment: "true",
        };
        let transaction_builder_params = TransactionBuilderParams::AddBid {
            public_key: PublicKey::from_hex(SAMPLE_ACCOUNT).unwrap(),
            delegation_rate: 0,
            amount: U512::from(10),
            minimum_delegation_amount,
            maximum_delegation_amount,
        };
        let transaction =
            create_transaction(transaction_builder_params, transaction_string_params, false);
        assert!(transaction.is_err(), "{:?}", transaction);
        println!("{:?}", transaction);
        assert!(matches!(
            transaction.unwrap_err(),
            CliError::InvalidArgument {
                context: "create_transaction (secret_key, allow_unsigned_deploy)",
                error: _
            }
        ));
    }
}
