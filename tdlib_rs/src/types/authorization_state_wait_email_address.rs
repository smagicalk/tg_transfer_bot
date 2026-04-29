#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// TDLib needs the user's email address to authorize. Call setAuthenticationEmailAddress to provide the email address, or directly call checkAuthenticationEmailCode with Apple ID/Google ID token if allowed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthorizationStateWaitEmailAddress {
    /// True, if authorization through Apple ID is allowed
    pub allow_apple_id: bool,
    /// True, if authorization through Google ID is allowed
    pub allow_google_id: bool,
}
