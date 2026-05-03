#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Information about the authentication code that was sent
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AuthenticationCodeInfo {
    /// A phone number that is being authenticated
    pub phone_number: String,
    /// The way the code was sent to the user
    pub r#type: crate::enums::AuthenticationCodeType,
    /// The way the next code will be sent to the user; may be null
    pub next_type: Option<crate::enums::AuthenticationCodeType>,
    /// Timeout before the code can be re-sent, in seconds
    pub timeout: i32,
}
