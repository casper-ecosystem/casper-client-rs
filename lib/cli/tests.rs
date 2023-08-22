use casper_types::{AsymmetricType, PublicKey, SecretKey};

use crate::{
    types::{ExecutableDeployItem, MAX_SERIALIZED_SIZE_OF_DEPLOY},
    Error, OutputKind,
};

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
        PaymentStrParams::with_package_hash(PKG_HASH, VERSION, ENTRYPOINT, args_simple(), "", "");
    let session_params =
        SessionStrParams::with_package_hash(PKG_HASH, VERSION, ENTRYPOINT, args_simple(), "", "");

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
        PaymentStrParams::with_package_hash(PKG_HASH, VERSION, ENTRYPOINT, args_simple(), "", "");
    // Create a string arg of 1048576 letter 'a's to ensure the deploy is greater than 1048576
    // bytes.
    let large_args_simple = format!("name_01:string='{:a<1048576}'", "");

    let session_params = SessionStrParams::with_package_hash(
        PKG_HASH,
        VERSION,
        ENTRYPOINT,
        vec![large_args_simple.as_str()],
        "",
        "",
    );

    match deploy::with_payment_and_session(deploy_params, payment_params, session_params, false) {
        Err(CliError::Core(Error::DeploySizeTooLarge {
            max_deploy_size,
            actual_deploy_size,
        })) => {
            assert_eq!(max_deploy_size, MAX_SERIALIZED_SIZE_OF_DEPLOY);
            assert!(actual_deploy_size > MAX_SERIALIZED_SIZE_OF_DEPLOY as usize);
        }
        Err(error) => panic!("unexpected error: {}", error),
        Ok(_) => panic!("failed to error while creating an excessively large deploy"),
    }
}

#[test]
fn should_read_deploy() {
    let bytes = SAMPLE_DEPLOY.as_bytes();
    assert!(matches!(crate::read_deploy(bytes), Ok(_)));
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
        Vec::new(),
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
        Vec::new(),
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
        Vec::new(),
    );

    assert!(transfer_deploy.is_ok());
    assert!(matches!(
        transfer_deploy.unwrap().session(),
        ExecutableDeployItem::Transfer { .. }
    ));
}

#[test]
fn should_create_transfer_with_custom_args() {
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
        vec!["targetAccountHex:public_key='012bac1d0ff9240ff0b7b06d555815640497861619ca12583ddef434885416e69b'"],
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
        Vec::new(),
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
        PaymentStrParams::with_package_hash(PKG_HASH, VERSION, ENTRYPOINT, args_simple(), "", "");
    let session_params =
        SessionStrParams::with_package_hash(PKG_HASH, VERSION, ENTRYPOINT, args_simple(), "", "");

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
        PaymentStrParams::with_package_hash(PKG_HASH, VERSION, ENTRYPOINT, args_simple(), "", "");
    let session_params =
        SessionStrParams::with_package_hash(PKG_HASH, VERSION, ENTRYPOINT, args_simple(), "", "");

    let deploy =
        deploy::with_payment_and_session(deploy_params, payment_params, session_params, true);
    assert!(deploy.is_err());
    assert!(matches!(
        deploy.unwrap_err(),
        CliError::Core(Error::DeployMissingSessionAccount)
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
        Vec::new(),
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
        Vec::new(),
    );
    assert!(transfer_deploy.is_err());
    assert!(matches!(
        transfer_deploy.unwrap_err(),
        CliError::Core(Error::DeployMissingSessionAccount)
    ));
}

#[test]
fn should_fail_to_create_transfer_with_no_secret_key_while_not_allowing_unsigned_deploy() {
    let deploy_params = deploy_params_without_secret_key();
    let payment_params =
        PaymentStrParams::with_package_hash(PKG_HASH, VERSION, ENTRYPOINT, args_simple(), "", "");

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
        Vec::new(),
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
        PaymentStrParams::with_package_hash(PKG_HASH, VERSION, ENTRYPOINT, args_simple(), "", "");
    let session_params =
        SessionStrParams::with_package_hash(PKG_HASH, VERSION, ENTRYPOINT, args_simple(), "", "");

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
