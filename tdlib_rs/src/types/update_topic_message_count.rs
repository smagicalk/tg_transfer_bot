#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Number of messages in a topic has changed; for Saved Messages and channel direct messages chat topics only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateTopicMessageCount {
    /// Identifier of the chat in topic of which the number of messages has changed
    pub chat_id: i64,
    /// Identifier of the topic
    pub topic_id: crate::enums::MessageTopic,
    /// Approximate number of messages in the topic
    pub message_count: i32,
}
