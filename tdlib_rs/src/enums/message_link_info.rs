#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageLinkInfo {
    /// Contains information about a link to a message or a forum topic in a chat
    #[serde(rename(serialize = "messageLinkInfo", deserialize = "messageLinkInfo"))]
    MessageLinkInfo(crate::types::MessageLinkInfo),
}
