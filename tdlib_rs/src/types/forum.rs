//!
//! TDLib `forum` domain types.
//!
//! Types, enums, and functions for supergroup forum topics and topic management.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A topic in a non-forum supergroup chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageTopicThread {
    /// Unique identifier of the message thread
    pub message_thread_id: i64,
}

/// A topic in a forum supergroup chat or a chat with a bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageTopicForum {
    /// Unique identifier of the forum topic
    pub forum_topic_id: i32,
}

/// A topic in a channel direct messages chat administered by the current user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageTopicDirectMessages {
    /// Unique identifier of the topic
    pub direct_messages_chat_topic_id: i64,
}

/// A topic in Saved Messages chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageTopicSavedMessages {
    /// Unique identifier of the Saved Messages topic
    pub saved_messages_topic_id: i64,
}

/// Topic containing messages sent by the current user of forwarded from an unknown chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SavedMessagesTopicTypeMyNotes {
}

/// Topic containing messages forwarded from a user with hidden privacy
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SavedMessagesTopicTypeAuthorHidden {
}

/// Topic containing messages forwarded from a specific chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SavedMessagesTopicTypeSavedFromChat {
    /// Identifier of the chat
    pub chat_id: i64,
}

/// Contains information about a Saved Messages topic
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SavedMessagesTopic {
    /// Unique topic identifier
    pub id: i64,
    /// Type of the topic
    #[serde(rename = "type")]
    pub r#type: crate::enums::SavedMessagesTopicType,
    /// True, if the topic is pinned
    pub is_pinned: bool,
    /// A parameter used to determine order of the topic in the topic list. Topics must be sorted by the order in descending order
    #[serde_as(as = "DisplayFromStr")]
    pub order: i64,
    /// Last message in the topic; may be null if none or unknown
    pub last_message: Option<crate::types::Message>,
    /// A draft of a message in the topic; may be null if none
    pub draft_message: Option<crate::types::DraftMessage>,
}

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

/// Describes a forum topic icon
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ForumTopicIcon {
    /// Color of the topic icon in RGB format
    pub color: i32,
    /// Unique identifier of the custom emoji shown on the topic icon; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub custom_emoji_id: i64,
}

/// Contains basic information about a forum topic
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ForumTopicInfo {
    /// Identifier of a forum supergroup chat or a chat with a bot to which the topic belongs
    pub chat_id: i64,
    /// Forum topic identifier of the topic
    pub forum_topic_id: i32,
    /// Name of the topic
    pub name: String,
    /// Icon of the topic
    pub icon: crate::types::ForumTopicIcon,
    /// Point in time (Unix timestamp) when the topic was created
    pub creation_date: i32,
    /// Identifier of the creator of the topic
    pub creator_id: crate::enums::MessageSender,
    /// True, if the topic is the General topic
    pub is_general: bool,
    /// True, if the topic was created by the current user
    pub is_outgoing: bool,
    /// True, if the topic is closed. If the topic is closed, then the user must have can_manage_topics administrator right in the supergroup or must be the creator of the topic to send messages there
    pub is_closed: bool,
    /// True, if the topic is hidden above the topic list and closed; for General topic only
    pub is_hidden: bool,
    /// True, if the name of the topic wasn't added explicitly
    pub is_name_implicit: bool,
}

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
    /// Number of messages with unread poll votes in the topic
    pub unread_poll_vote_count: i32,
    /// Notification settings for the topic
    pub notification_settings: crate::types::ChatNotificationSettings,
    /// A draft of a message in the topic; may be null if none
    pub draft_message: Option<crate::types::DraftMessage>,
}

/// Describes a list of forum topics
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ForumTopics {
    /// Approximate total number of forum topics found
    pub total_count: i32,
    /// List of forum topics
    pub topics: Vec<crate::types::ForumTopic>,
    /// Offset date for the next getForumTopics request
    pub next_offset_date: i32,
    /// Offset message identifier for the next getForumTopics request
    pub next_offset_message_id: i64,
    /// Offset forum topic identifier for the next getForumTopics request
    pub next_offset_forum_topic_id: i32,
}

/// A forum topic has been created
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageForumTopicCreated {
    /// Name of the topic
    pub name: String,
    /// True, if the name of the topic wasn't added explicitly
    pub is_name_implicit: bool,
    /// Icon of the topic
    pub icon: crate::types::ForumTopicIcon,
}

/// A forum topic has been edited
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageForumTopicEdited {
    /// If non-empty, the new name of the topic
    pub name: String,
    /// True, if icon's custom_emoji_id is changed
    pub edit_icon_custom_emoji_id: bool,
    /// New unique identifier of the custom emoji shown on the topic icon; 0 if none. Must be ignored if edit_icon_custom_emoji_id is false
    #[serde_as(as = "DisplayFromStr")]
    pub icon_custom_emoji_id: i64,
}

/// A forum topic has been closed or opened
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageForumTopicIsClosedToggled {
    /// True, if the topic was closed; otherwise, the topic was reopened
    pub is_closed: bool,
}

/// A General forum topic has been hidden or unhidden
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageForumTopicIsHiddenToggled {
    /// True, if the topic was hidden; otherwise, the topic was unhidden
    pub is_hidden: bool,
}

/// The is_forum setting of a supergroup was toggled
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventIsForumToggled {
    /// New value of is_forum
    pub is_forum: bool,
}

/// A new forum topic was created
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventForumTopicCreated {
    /// Information about the topic
    pub topic_info: crate::types::ForumTopicInfo,
}

/// A forum topic was edited
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventForumTopicEdited {
    /// Old information about the topic
    pub old_topic_info: crate::types::ForumTopicInfo,
    /// New information about the topic
    pub new_topic_info: crate::types::ForumTopicInfo,
}

/// A forum topic was closed or reopened
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventForumTopicToggleIsClosed {
    /// New information about the topic
    pub topic_info: crate::types::ForumTopicInfo,
}

/// The General forum topic was hidden or unhidden
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventForumTopicToggleIsHidden {
    /// New information about the topic
    pub topic_info: crate::types::ForumTopicInfo,
}

/// A forum topic was deleted
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventForumTopicDeleted {
    /// Information about the topic
    pub topic_info: crate::types::ForumTopicInfo,
}

/// A pinned forum topic was changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventForumTopicPinned {
    /// Information about the old pinned topic; may be null
    pub old_topic_info: Option<crate::types::ForumTopicInfo>,
    /// Information about the new pinned topic; may be null
    pub new_topic_info: Option<crate::types::ForumTopicInfo>,
}

/// A chat default appearance has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatViewAsTopics {
    /// Chat identifier
    pub chat_id: i64,
    /// New value of view_as_topics
    pub view_as_topics: bool,
}

/// Basic information about a Saved Messages topic has changed. This update is guaranteed to come before the topic identifier is returned to the application
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateSavedMessagesTopic {
    /// New data about the topic
    pub topic: crate::types::SavedMessagesTopic,
}

/// Number of Saved Messages topics has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateSavedMessagesTopicCount {
    /// Approximate total number of Saved Messages topics
    pub topic_count: i32,
}

/// Basic information about a topic in a channel direct messages chat administered by the current user has changed. This update is guaranteed to come before the topic identifier is returned to the application
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateDirectMessagesChatTopic {
    /// New data about the topic
    pub topic: crate::types::DirectMessagesChatTopic,
}

/// Number of messages in a topic has changed; for Saved Messages and channel direct messages chat topics only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateTopicMessageCount {
    /// Identifier of the chat in topic of which the number of messages has changed
    pub chat_id: i64,
    /// Identifier of the topic
    pub topic_id: crate::enums::MessageTopic,
    /// Approximate number of messages in the topic
    pub message_count: i32,
}

/// Basic information about a topic in a forum chat was changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateForumTopicInfo {
    /// New information about the topic
    pub info: crate::types::ForumTopicInfo,
}

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
    /// Number of messages with unread poll votes in the topic
    pub unread_poll_vote_count: i32,
    /// Notification settings for the topic
    pub notification_settings: crate::types::ChatNotificationSettings,
    /// A draft of a message in the topic; may be null if none
    pub draft_message: Option<crate::types::DraftMessage>,
}

