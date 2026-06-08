#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatBoostLinkInfo {
    /// Contains information about a link to boost a chat
    #[serde(rename(serialize = "chatBoostLinkInfo", deserialize = "chatBoostLinkInfo"))]
    ChatBoostLinkInfo(crate::types::ChatBoostLinkInfo),
}
