#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Device verification must be performed with the SafetyNet Attestation API
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FirebaseDeviceVerificationParametersSafetyNet {
    /// Nonce to pass to the SafetyNet Attestation API
    pub nonce: String,
}
