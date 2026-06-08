#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A topic in a forum supergroup chat or a chat with a bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageTopicForum {
    /// Unique identifier of the forum topic
    pub forum_topic_id: i32,
}
