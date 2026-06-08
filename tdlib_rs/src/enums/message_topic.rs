#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageTopic {
    /// A topic in a non-forum supergroup chat
    #[serde(rename(serialize = "messageTopicThread", deserialize = "messageTopicThread"))]
    Thread(crate::types::MessageTopicThread),
    /// A topic in a forum supergroup chat or a chat with a bot
    #[serde(rename(serialize = "messageTopicForum", deserialize = "messageTopicForum"))]
    Forum(crate::types::MessageTopicForum),
    /// A topic in a channel direct messages chat administered by the current user
    #[serde(rename(
        serialize = "messageTopicDirectMessages",
        deserialize = "messageTopicDirectMessages"
    ))]
    DirectMessages(crate::types::MessageTopicDirectMessages),
    /// A topic in Saved Messages chat
    #[serde(rename(
        serialize = "messageTopicSavedMessages",
        deserialize = "messageTopicSavedMessages"
    ))]
    SavedMessages(crate::types::MessageTopicSavedMessages),
}
