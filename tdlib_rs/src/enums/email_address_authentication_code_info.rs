#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmailAddressAuthenticationCodeInfo {
    /// Information about the email address authentication code that was sent
    #[serde(rename(serialize = "emailAddressAuthenticationCodeInfo", deserialize = "emailAddressAuthenticationCodeInfo"))]
    EmailAddressAuthenticationCodeInfo(crate::types::EmailAddressAuthenticationCodeInfo),
}
