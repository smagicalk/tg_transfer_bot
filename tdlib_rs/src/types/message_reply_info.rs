#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about replies to a message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageReplyInfo {
    /// Number of times the message was directly or indirectly replied
    pub reply_count: i32,
    /// Identifiers of at most 3 recent repliers to the message; available in channels with a discussion supergroup. The users and chats are expected to be inaccessible: only their photo and name will be available
    pub recent_replier_ids: Vec<crate::enums::MessageSender>,
    /// Identifier of the last read incoming reply to the message
    pub last_read_inbox_message_id: i64,
    /// Identifier of the last read outgoing reply to the message
    pub last_read_outbox_message_id: i64,
    /// Identifier of the last reply to the message
    pub last_message_id: i64,
}
