#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmailAddressAuthentication {
    /// An authentication code delivered to a user's email address
    #[serde(rename(serialize = "emailAddressAuthenticationCode", deserialize = "emailAddressAuthenticationCode"))]
    Code(crate::types::EmailAddressAuthenticationCode),
    /// An authentication token received through Apple ID
    #[serde(rename(serialize = "emailAddressAuthenticationAppleId", deserialize = "emailAddressAuthenticationAppleId"))]
    AppleId(crate::types::EmailAddressAuthenticationAppleId),
    /// An authentication token received through Google ID
    #[serde(rename(serialize = "emailAddressAuthenticationGoogleId", deserialize = "emailAddressAuthenticationGoogleId"))]
    GoogleId(crate::types::EmailAddressAuthenticationGoogleId),
}
