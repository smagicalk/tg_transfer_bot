#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The code is re-sent, because device verification has failed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ResendCodeReasonVerificationFailed {
    /// Cause of the verification failure, for example, "PLAY_SERVICES_NOT_AVAILABLE", "APNS_RECEIVE_TIMEOUT", or "APNS_INIT_FAILED"
    pub error_message: String,
}
