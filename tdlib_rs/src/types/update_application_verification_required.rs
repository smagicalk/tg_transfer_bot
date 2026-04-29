#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A request can't be completed unless application verification is performed; for official mobile applications only.
/// The method setApplicationVerificationToken must be called once the verification is completed or failed
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateApplicationVerificationRequired {
    /// Unique identifier for the verification process
    pub verification_id: i64,
    /// Unique base64url-encoded nonce for the classic Play Integrity verification (https:developer.android.com/google/play/integrity/classic) for Android,
    /// or a unique string to compare with verify_nonce field from a push notification for iOS
    pub nonce: String,
    /// Cloud project number to pass to the Play Integrity API on Android
    #[serde_as(as = "DisplayFromStr")]
    pub cloud_project_number: i64,
}
