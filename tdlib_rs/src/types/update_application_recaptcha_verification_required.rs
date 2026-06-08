#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A request can't be completed unless reCAPTCHA verification is performed; for official mobile applications only.
/// The method setApplicationVerificationToken must be called once the verification is completed or failed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateApplicationRecaptchaVerificationRequired {
    /// Unique identifier for the verification process
    pub verification_id: i64,
    /// The action for the check
    pub action: String,
    /// Identifier of the reCAPTCHA key
    pub recaptcha_key_id: String,
}
