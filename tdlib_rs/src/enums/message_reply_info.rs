#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageReplyInfo {
    /// Contains information about replies to a message
    #[serde(rename(serialize = "messageReplyInfo", deserialize = "messageReplyInfo"))]
    MessageReplyInfo(crate::types::MessageReplyInfo),
}
