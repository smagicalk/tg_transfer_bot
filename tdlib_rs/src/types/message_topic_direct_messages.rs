#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A topic in a channel direct messages chat administered by the current user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageTopicDirectMessages {
    /// Unique identifier of the topic
    pub direct_messages_chat_topic_id: i64,
}
