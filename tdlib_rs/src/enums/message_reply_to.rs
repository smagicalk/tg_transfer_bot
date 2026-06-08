#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageReplyTo {
    /// Describes a message replied by a given message
    #[serde(rename(
        serialize = "messageReplyToMessage",
        deserialize = "messageReplyToMessage"
    ))]
    Message(crate::types::MessageReplyToMessage),
    /// Describes a story replied by a given message
    #[serde(rename(serialize = "messageReplyToStory", deserialize = "messageReplyToStory"))]
    Story(crate::types::MessageReplyToStory),
}
