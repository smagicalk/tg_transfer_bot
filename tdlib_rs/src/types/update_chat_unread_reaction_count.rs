#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The chat unread_reaction_count has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatUnreadReactionCount {
    /// Chat identifier
    pub chat_id: i64,
    /// The number of messages with unread reactions left in the chat
    pub unread_reaction_count: i32,
}
