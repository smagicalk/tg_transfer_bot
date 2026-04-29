#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Information about the email address authentication code that was sent
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmailAddressAuthenticationCodeInfo {
    /// Pattern of the email address to which an authentication code was sent
    pub email_address_pattern: String,
    /// Length of the code; 0 if unknown
    pub length: i32,
}
