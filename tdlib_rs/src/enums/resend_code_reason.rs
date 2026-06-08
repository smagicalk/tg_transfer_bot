#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ResendCodeReason {
    /// The user requested to resend the code
    #[serde(rename(
        serialize = "resendCodeReasonUserRequest",
        deserialize = "resendCodeReasonUserRequest"
    ))]
    UserRequest,
    /// The code is re-sent, because device verification has failed
    #[serde(rename(
        serialize = "resendCodeReasonVerificationFailed",
        deserialize = "resendCodeReasonVerificationFailed"
    ))]
    VerificationFailed(crate::types::ResendCodeReasonVerificationFailed),
}
