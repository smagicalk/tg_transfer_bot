#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about a message thread
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageThreadInfo {
    /// Identifier of the chat to which the message thread belongs
    pub chat_id: i64,
    /// Message thread identifier, unique within the chat
    pub message_thread_id: i64,
    /// Information about the message thread; may be null for forum topic threads
    pub reply_info: Option<crate::types::MessageReplyInfo>,
    /// Approximate number of unread messages in the message thread
    pub unread_message_count: i32,
    /// The messages from which the thread starts. The messages are returned in reverse chronological order (i.e., in order of decreasing message_id)
    pub messages: Vec<crate::types::Message>,
    /// A draft of a message in the message thread; may be null if none
    pub draft_message: Option<crate::types::DraftMessage>,
}
