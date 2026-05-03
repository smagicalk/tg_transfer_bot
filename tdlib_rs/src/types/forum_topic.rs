#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a forum topic
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ForumTopic {
    /// Basic information about the topic
    pub info: crate::types::ForumTopicInfo,
    /// Last message in the topic; may be null if unknown
    pub last_message: Option<crate::types::Message>,
    /// A parameter used to determine order of the topic in the topic list. Topics must be sorted by the order in descending order
    #[serde_as(as = "DisplayFromStr")]
    pub order: i64,
    /// True, if the topic is pinned in the topic list
    pub is_pinned: bool,
    /// Number of unread messages in the topic
    pub unread_count: i32,
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
