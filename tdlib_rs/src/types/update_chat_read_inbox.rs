#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Incoming messages were read or the number of unread messages has been changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatReadInbox {
    /// Chat identifier
    pub chat_id: i64,
    /// Identifier of the last read incoming message
    pub last_read_inbox_message_id: i64,
    /// The number of unread messages left in the chat
    pub unread_count: i32,
}
