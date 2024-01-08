use std::path::Path;

use bytes::{BufMut, Bytes, BytesMut};
use casper_types::PublicKey;
use flate2::{write::GzEncoder, Compression};
use openapi::{
    apis::{
        configuration::Configuration,
        default_api::{
            verification_deploy_hash_details_get, verification_deploy_hash_status_get,
            verification_post,
        },
    },
    models::{VerificationDetails, VerificationRequest, VerificationStatus},
};
use tar::Builder as TarBuilder;
use tokio::time::{sleep, Duration};

use crate::{types::DeployHash, Error, Verbosity};

static GIT_DIR_NAME: &str = ".git";
static TARGET_DIR_NAME: &str = "target";

/// Builds an archive from the specified path.
///
/// This function creates a compressed tar archive from the files and directories located at the
/// specified path. It excludes the `.git` and `target` directories from the archive.
///
/// # Arguments
///
/// * `path` - The path to the directory containing the files and directories to be archived.
///
/// # Returns
///
/// The compressed tar archive as a `Bytes` object, or an `std::io::Error` if an error occurs during
/// the archiving process.
pub fn build_archive(path: &Path) -> Result<Bytes, std::io::Error> {
    let buffer = BytesMut::new().writer();
    let encoder = GzEncoder::new(buffer, Compression::best());
    let mut archive = TarBuilder::new(encoder);

    for entry in (path.read_dir()?).flatten() {
        let file_name = entry.file_name();
        // Skip `.git` and `target`.
        if file_name == TARGET_DIR_NAME || file_name == GIT_DIR_NAME {
            continue;
        }
        let full_path = entry.path();
        if full_path.is_dir() {
            archive.append_dir_all(&file_name, &full_path)?;
        } else {
            archive.append_path_with_name(&full_path, &file_name)?;
        }
    }

    let encoder = archive.into_inner()?;
    let buffer = encoder.finish()?;
    Ok(buffer.into_inner().freeze())
}

/// Verifies the smart contract code against the one deployed at deploy hash.
///
/// Sends a verification request to the specified verification URL base path, including the deploy hash,
/// public key, and code archive.
///
/// # Arguments
///
/// * `deploy_hash` - The hash of the deployed contract.
/// * `public_key` - The public key associated with the contract.
/// * `verification_url_base_path` - The base path of the verification URL.
/// * `verbosity` - The verbosity level of the verification process.
///
/// # Returns
///
/// The verification details of the contract.
pub async fn send_verification_request(
    deploy_hash: DeployHash,
    public_key: PublicKey,
    verification_url_base_path: &str,
    archive_base64: String,
    verbosity: Verbosity,
) -> Result<VerificationDetails, Error> {
    let verification_request = VerificationRequest {
        deploy_hash: Some(deploy_hash.to_string()),
        public_key: Some(public_key.to_account_hash().to_string()),
        code_archive: Some(archive_base64),
    };

    let mut configuration = Configuration::default();
    configuration.base_path = verification_url_base_path.to_string();

    if verbosity == Verbosity::Medium || verbosity == Verbosity::High {
        println!("Sending verfication request to {}", configuration.base_path);
    }

    let verification_result =
        match verification_post(&configuration, Some(verification_request)).await {
            Ok(verification_result) => verification_result,
            Err(error) => {
                eprintln!("Cannot send verification request: {:?}", error);
                return Err(Error::ContractVerificationFailed);
            }
        };

    if verbosity == Verbosity::Medium || verbosity == Verbosity::High {
        println!(
            "Sent verification request - status {}",
            verification_result.status.unwrap().to_string()
        );
    }

    wait_for_verification_finished(&configuration, deploy_hash, verbosity).await;

    if verbosity == Verbosity::Medium || verbosity == Verbosity::High {
        println!("Getting verification details...");
    }

    match verification_deploy_hash_details_get(&configuration, deploy_hash.to_string().as_str())
        .await
    {
        Ok(verification_details) => Ok(verification_details),
        Err(error) => {
            eprintln!("Cannot get verification details: {:?}", error);
            Err(Error::ContractVerificationFailed)
        }
    }
}

async fn wait_for_verification_finished(
    configuration: &Configuration,
    deploy_hash: DeployHash,
    verbosity: Verbosity,
) {
    let mut verification_status = match get_verification_status(configuration, deploy_hash).await {
        Ok(verification_status) => verification_status,
        Err(error) => {
            eprintln!("Cannot get verification status: {:?}", error);
            return;
        }
    };

    while verification_status != VerificationStatus::Verified
        && verification_status != VerificationStatus::Failed
    {
        verification_status = match get_verification_status(configuration, deploy_hash).await {
            Ok(verification_status) => verification_status,
            Err(error) => {
                eprintln!("Cannot get verification status: {:?}", error);
                return;
            }
        };

        sleep(Duration::from_millis(100)).await;
        // TODO: Add backoff with limited retries.
    }

    if verbosity == Verbosity::Medium || verbosity == Verbosity::High {
        println!(
            "Verification finished - status {}",
            verification_status.to_string()
        );
    }
}

async fn get_verification_status(
    configuration: &Configuration,
    deploy_hash: DeployHash,
) -> Result<VerificationStatus, Error> {
    let verification_status =
        match verification_deploy_hash_status_get(&configuration, deploy_hash.to_string().as_str())
            .await
        {
            Ok(verification_result) => verification_result,
            Err(error) => {
                eprintln!("Failed to fetch verification status: {:?}", error);
                return Err(Error::ContractVerificationFailed);
            }
        };

    match verification_status.status {
        Some(verification_status) => Ok(verification_status),
        None => {
            eprintln!("Verification status not found");
            Err(Error::ContractVerificationFailed)
        }
    }
}
