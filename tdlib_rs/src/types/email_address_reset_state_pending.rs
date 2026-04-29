#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Email address reset has already been requested. Call resetAuthenticationEmailAddress to check whether immediate reset is possible
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmailAddressResetStatePending {
    /// Left time before the email address will be reset, in seconds. updateAuthorizationState is not sent when this field changes
    pub reset_in: i32,
}
