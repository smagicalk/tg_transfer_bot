#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Information about a topic in a forum chat was changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateForumTopic {
    /// Chat identifier
    pub chat_id: i64,
    /// Forum topic identifier of the topic
    pub forum_topic_id: i32,
    /// True, if the topic is pinned in the topic list
    pub is_pinned: bool,
    /// Identifier of the last read incoming message
    pub last_read_inbox_message_id: i64,
    /// Identifier of the last read outgoing message
    pub last_read_outbox_message_id: i64,
    /// Number of unread messages with a mention/reply in the topic
    pub unread_mention_count: i32,
    /// Number of messages with unread reactions in the topic
    pub unread_reaction_count: i32,
    /// Notification settings for the topic
    pub notification_settings: crate::types::ChatNotificationSettings,
    /// A draft of a message in the topic; may be null if none
    pub draft_message: Option<crate::types::DraftMessage>,
}
