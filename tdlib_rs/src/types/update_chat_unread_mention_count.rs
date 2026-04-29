#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The chat unread_mention_count has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatUnreadMentionCount {
    /// Chat identifier
    pub chat_id: i64,
    /// The number of unread mention messages left in the chat
    pub unread_mention_count: i32,
}
