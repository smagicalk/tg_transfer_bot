#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PublicForward {
    /// Contains a public forward as a message
    #[serde(rename(
        serialize = "publicForwardMessage",
        deserialize = "publicForwardMessage"
    ))]
    Message(crate::types::PublicForwardMessage),
    /// Contains a public repost to a story
    #[serde(rename(serialize = "publicForwardStory", deserialize = "publicForwardStory"))]
    Story(crate::types::PublicForwardStory),
}
