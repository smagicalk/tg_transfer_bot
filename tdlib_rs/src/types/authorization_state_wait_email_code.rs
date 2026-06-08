#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// TDLib needs the user's authentication code sent to an email address to authorize. Call checkAuthenticationEmailCode to provide the code
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthorizationStateWaitEmailCode {
    /// True, if authorization through Apple ID is allowed
    pub allow_apple_id: bool,
    /// True, if authorization through Google ID is allowed
    pub allow_google_id: bool,
    /// Information about the sent authentication code
    pub code_info: crate::types::EmailAddressAuthenticationCodeInfo,
    /// Reset state of the email address; may be null if the email address can't be reset
    pub email_address_reset_state: Option<crate::enums::EmailAddressResetState>,
}
