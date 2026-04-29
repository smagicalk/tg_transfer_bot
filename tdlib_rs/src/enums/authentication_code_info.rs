#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AuthenticationCodeInfo {
    /// Information about the authentication code that was sent
    #[serde(rename(serialize = "authenticationCodeInfo", deserialize = "authenticationCodeInfo"))]
    AuthenticationCodeInfo(crate::types::AuthenticationCodeInfo),
}
