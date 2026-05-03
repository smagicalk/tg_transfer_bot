#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Device verification must be performed with the classic Play Integrity verification (https:developer.android.com/google/play/integrity/classic)
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FirebaseDeviceVerificationParametersPlayIntegrity {
    /// Base64url-encoded nonce to pass to the Play Integrity API
    pub nonce: String,
    /// Cloud project number to pass to the Play Integrity API
    #[serde_as(as = "DisplayFromStr")]
    pub cloud_project_number: i64,
}
