use casper_types::Key;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub(crate) enum VerificationStatus {
    Failed,
    Pending,
    Verified,
    Waiting,
}

// Any update to this enum should be reflected in migrations.
#[derive(Deserialize, Serialize)]
#[non_exhaustive]
pub(crate) enum VerificationErrorCode {
    Ok,
    None,
    BytecodeMismatch,
    BuildError,
    ContractAlreadyVerified,
    ContractNotFound,
    Internal,
    InvalidContract,
    InvalidHash,
    WrongCodeArchive,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct VerificationError {
    pub(crate) code: VerificationErrorCode,
    pub(crate) description: String,
}

#[derive(Serialize)]
pub(crate) struct VerificationRequest {
    // Deploy hash of the contract deployment transaction.
    pub(crate) deploy_hash: Key,
    // Base64 encoded tar archive containing contract source code.
    pub(crate) code_archive: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct VerificationResult {
    pub(crate) status: VerificationStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) error: Option<VerificationError>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct VerificationDetailsResult {
    // pub(crate) toolchain: Toolchain,
    pub(crate) binary_uri: String,
    pub(crate) logs_uri: String,
}

#[derive(Deserialize, Serialize)]
pub struct VerificationDetails {
    pub(crate) status: VerificationStatus,
    pub(crate) result: VerificationDetailsResult,
}
