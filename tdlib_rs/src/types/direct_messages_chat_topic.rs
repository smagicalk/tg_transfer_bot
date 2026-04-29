#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about a topic in a channel direct messages chat administered by the current user
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DirectMessagesChatTopic {
    /// Identifier of the chat to which the topic belongs
    pub chat_id: i64,
    /// Unique topic identifier
    pub id: i64,
    /// Identifier of the user or chat that sends the messages to the topic
    pub sender_id: crate::enums::MessageSender,
    /// A parameter used to determine order of the topic in the topic list. Topics must be sorted by the order in descending order
    #[serde_as(as = "DisplayFromStr")]
    pub order: i64,
    /// True, if the other party can send unpaid messages even if the chat has paid messages enabled
    pub can_send_unpaid_messages: bool,
    /// True, if the topic is marked as unread
    pub is_marked_as_unread: bool,
    /// Number of unread messages in the chat
    pub unread_count: i64,
    /// Identifier of the last read incoming message
    pub last_read_inbox_message_id: i64,
    /// Identifier of the last read outgoing message
    pub last_read_outbox_message_id: i64,
    /// Number of messages with unread reactions in the chat
    pub unread_reaction_count: i64,
    /// Last message in the topic; may be null if none or unknown
    pub last_message: Option<crate::types::Message>,
    /// A draft of a message in the topic; may be null if none
    pub draft_message: Option<crate::types::DraftMessage>,
}
