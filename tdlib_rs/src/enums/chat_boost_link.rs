#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatBoostLink {
    /// Contains an HTTPS link to boost a chat
    #[serde(rename(serialize = "chatBoostLink", deserialize = "chatBoostLink"))]
    ChatBoostLink(crate::types::ChatBoostLink),
}
