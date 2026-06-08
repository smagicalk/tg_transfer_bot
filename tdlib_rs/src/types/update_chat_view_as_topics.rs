#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A chat default appearance has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatViewAsTopics {
    /// Chat identifier
    pub chat_id: i64,
    /// New value of view_as_topics
    pub view_as_topics: bool,
}
