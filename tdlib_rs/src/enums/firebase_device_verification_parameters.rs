#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FirebaseDeviceVerificationParameters {
    /// Device verification must be performed with the SafetyNet Attestation API
    #[serde(rename(
        serialize = "firebaseDeviceVerificationParametersSafetyNet",
        deserialize = "firebaseDeviceVerificationParametersSafetyNet"
    ))]
    SafetyNet(crate::types::FirebaseDeviceVerificationParametersSafetyNet),
    /// Device verification must be performed with the classic Play Integrity verification (https:developer.android.com/google/play/integrity/classic)
    #[serde(rename(
        serialize = "firebaseDeviceVerificationParametersPlayIntegrity",
        deserialize = "firebaseDeviceVerificationParametersPlayIntegrity"
    ))]
    PlayIntegrity(crate::types::FirebaseDeviceVerificationParametersPlayIntegrity),
}
