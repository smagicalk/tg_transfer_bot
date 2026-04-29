#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The list of unread reactions added to a message was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageUnreadReactions {
    /// Chat identifier
    pub chat_id: i64,
    /// Message identifier
    pub message_id: i64,
    /// The new list of unread reactions
    pub unread_reactions: Vec<crate::types::UnreadReaction>,
    /// The new number of messages with unread reactions left in the chat
    pub unread_reaction_count: i32,
}
