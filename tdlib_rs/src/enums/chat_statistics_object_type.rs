#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatStatisticsObjectType {
    /// Describes a message sent in the chat
    #[serde(rename(
        serialize = "chatStatisticsObjectTypeMessage",
        deserialize = "chatStatisticsObjectTypeMessage"
    ))]
    Message(crate::types::ChatStatisticsObjectTypeMessage),
    /// Describes a story posted on behalf of the chat
    #[serde(rename(
        serialize = "chatStatisticsObjectTypeStory",
        deserialize = "chatStatisticsObjectTypeStory"
    ))]
    Story(crate::types::ChatStatisticsObjectTypeStory),
}
