#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// TDLib needs the user's authentication code to authorize. Call checkAuthenticationCode to check the code
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AuthorizationStateWaitCode {
    /// Information about the authorization code that was sent
    pub code_info: crate::types::AuthenticationCodeInfo,
}
