#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


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
