#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageFileType {
    /// The messages were exported from a private chat
    #[serde(rename(serialize = "messageFileTypePrivate", deserialize = "messageFileTypePrivate"))]
    Private(crate::types::MessageFileTypePrivate),
    /// The messages were exported from a group chat
    #[serde(rename(serialize = "messageFileTypeGroup", deserialize = "messageFileTypeGroup"))]
    Group(crate::types::MessageFileTypeGroup),
    /// The messages were exported from a chat of unknown type
    #[serde(rename(serialize = "messageFileTypeUnknown", deserialize = "messageFileTypeUnknown"))]
    Unknown,
}
