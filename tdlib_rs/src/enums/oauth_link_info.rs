#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum OauthLinkInfo {
    /// Information about the OAuth authorization
    #[serde(rename(serialize = "oauthLinkInfo", deserialize = "oauthLinkInfo"))]
    OauthLinkInfo(crate::types::OauthLinkInfo),
}
