#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum QuickReplyMessage {
    /// Describes a message that can be used for quick reply
    #[serde(rename(serialize = "quickReplyMessage", deserialize = "quickReplyMessage"))]
    QuickReplyMessage(crate::types::QuickReplyMessage),
}
