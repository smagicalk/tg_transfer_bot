//!
//! TDLib `forum` domain enums.
//!
//! Types, enums, and functions for supergroup forum topics and topic management.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

/// Describes a topic of messages in a chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageTopic {
    /// A topic in a non-forum supergroup chat
    #[serde(rename(serialize = "messageTopicThread", deserialize = "messageTopicThread"))]
    Thread(Box<crate::types::MessageTopicThread>),
    /// A topic in a forum supergroup chat or a chat with a bot
    #[serde(rename(serialize = "messageTopicForum", deserialize = "messageTopicForum"))]
    Forum(Box<crate::types::MessageTopicForum>),
    /// A topic in a channel direct messages chat administered by the current user
    #[serde(rename(serialize = "messageTopicDirectMessages", deserialize = "messageTopicDirectMessages"))]
    DirectMessages(Box<crate::types::MessageTopicDirectMessages>),
    /// A topic in Saved Messages chat
    #[serde(rename(serialize = "messageTopicSavedMessages", deserialize = "messageTopicSavedMessages"))]
    SavedMessages(Box<crate::types::MessageTopicSavedMessages>),
}

impl MessageTopic {
    /// Convenience constructor to create a [`MessageTopic::Thread`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn thread(val: crate::types::MessageTopicThread) -> Self {
        Self::Thread(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageTopic::Forum`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn forum(val: crate::types::MessageTopicForum) -> Self {
        Self::Forum(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageTopic::DirectMessages`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn direct_messages(val: crate::types::MessageTopicDirectMessages) -> Self {
        Self::DirectMessages(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageTopic::SavedMessages`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn saved_messages(val: crate::types::MessageTopicSavedMessages) -> Self {
        Self::SavedMessages(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageTopicThread`] into [`MessageTopic`].
impl From<crate::types::MessageTopicThread> for MessageTopic {
    fn from(val: crate::types::MessageTopicThread) -> Self {
        Self::Thread(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageTopicForum`] into [`MessageTopic`].
impl From<crate::types::MessageTopicForum> for MessageTopic {
    fn from(val: crate::types::MessageTopicForum) -> Self {
        Self::Forum(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageTopicDirectMessages`] into [`MessageTopic`].
impl From<crate::types::MessageTopicDirectMessages> for MessageTopic {
    fn from(val: crate::types::MessageTopicDirectMessages) -> Self {
        Self::DirectMessages(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageTopicSavedMessages`] into [`MessageTopic`].
impl From<crate::types::MessageTopicSavedMessages> for MessageTopic {
    fn from(val: crate::types::MessageTopicSavedMessages) -> Self {
        Self::SavedMessages(Box::new(val))
    }
}

/// Describes type of Saved Messages topic
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SavedMessagesTopicType {
    /// Topic containing messages sent by the current user of forwarded from an unknown chat
    #[serde(rename(serialize = "savedMessagesTopicTypeMyNotes", deserialize = "savedMessagesTopicTypeMyNotes"))]
    MyNotes,
    /// Topic containing messages forwarded from a user with hidden privacy
    #[serde(rename(serialize = "savedMessagesTopicTypeAuthorHidden", deserialize = "savedMessagesTopicTypeAuthorHidden"))]
    AuthorHidden,
    /// Topic containing messages forwarded from a specific chat
    #[serde(rename(serialize = "savedMessagesTopicTypeSavedFromChat", deserialize = "savedMessagesTopicTypeSavedFromChat"))]
    SavedFromChat(Box<crate::types::SavedMessagesTopicTypeSavedFromChat>),
}

impl SavedMessagesTopicType {
    /// Convenience constructor to create a [`SavedMessagesTopicType::SavedFromChat`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn saved_from_chat(val: crate::types::SavedMessagesTopicTypeSavedFromChat) -> Self {
        Self::SavedFromChat(Box::new(val))
    }

}

/// Converts a [`crate::types::SavedMessagesTopicTypeSavedFromChat`] into [`SavedMessagesTopicType`].
impl From<crate::types::SavedMessagesTopicTypeSavedFromChat> for SavedMessagesTopicType {
    fn from(val: crate::types::SavedMessagesTopicTypeSavedFromChat) -> Self {
        Self::SavedFromChat(Box::new(val))
    }
}

/// TDLib `SavedMessagesTopic` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SavedMessagesTopic {
    /// Contains information about a Saved Messages topic
    #[serde(rename(serialize = "savedMessagesTopic", deserialize = "savedMessagesTopic"))]
    SavedMessagesTopic(Box<crate::types::SavedMessagesTopic>),
}

impl SavedMessagesTopic {
    /// Convenience constructor to create a [`SavedMessagesTopic::SavedMessagesTopic`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn saved_messages_topic(val: crate::types::SavedMessagesTopic) -> Self {
        Self::SavedMessagesTopic(Box::new(val))
    }

}

/// Converts a [`crate::types::SavedMessagesTopic`] into [`SavedMessagesTopic`].
impl From<crate::types::SavedMessagesTopic> for SavedMessagesTopic {
    fn from(val: crate::types::SavedMessagesTopic) -> Self {
        Self::SavedMessagesTopic(Box::new(val))
    }
}

/// TDLib `DirectMessagesChatTopic` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum DirectMessagesChatTopic {
    /// Contains information about a topic in a channel direct messages chat administered by the current user
    #[serde(rename(serialize = "directMessagesChatTopic", deserialize = "directMessagesChatTopic"))]
    DirectMessagesChatTopic(Box<crate::types::DirectMessagesChatTopic>),
}

impl DirectMessagesChatTopic {
    /// Convenience constructor to create a [`DirectMessagesChatTopic::DirectMessagesChatTopic`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn direct_messages_chat_topic(val: crate::types::DirectMessagesChatTopic) -> Self {
        Self::DirectMessagesChatTopic(Box::new(val))
    }

}

/// Converts a [`crate::types::DirectMessagesChatTopic`] into [`DirectMessagesChatTopic`].
impl From<crate::types::DirectMessagesChatTopic> for DirectMessagesChatTopic {
    fn from(val: crate::types::DirectMessagesChatTopic) -> Self {
        Self::DirectMessagesChatTopic(Box::new(val))
    }
}

/// TDLib `ForumTopicIcon` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ForumTopicIcon {
    /// Describes a forum topic icon
    #[serde(rename(serialize = "forumTopicIcon", deserialize = "forumTopicIcon"))]
    ForumTopicIcon(Box<crate::types::ForumTopicIcon>),
}

impl ForumTopicIcon {
    /// Convenience constructor to create a [`ForumTopicIcon::ForumTopicIcon`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn forum_topic_icon(val: crate::types::ForumTopicIcon) -> Self {
        Self::ForumTopicIcon(Box::new(val))
    }

}

/// Converts a [`crate::types::ForumTopicIcon`] into [`ForumTopicIcon`].
impl From<crate::types::ForumTopicIcon> for ForumTopicIcon {
    fn from(val: crate::types::ForumTopicIcon) -> Self {
        Self::ForumTopicIcon(Box::new(val))
    }
}

/// TDLib `ForumTopicInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ForumTopicInfo {
    /// Contains basic information about a forum topic
    #[serde(rename(serialize = "forumTopicInfo", deserialize = "forumTopicInfo"))]
    ForumTopicInfo(Box<crate::types::ForumTopicInfo>),
}

impl ForumTopicInfo {
    /// Convenience constructor to create a [`ForumTopicInfo::ForumTopicInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn forum_topic_info(val: crate::types::ForumTopicInfo) -> Self {
        Self::ForumTopicInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::ForumTopicInfo`] into [`ForumTopicInfo`].
impl From<crate::types::ForumTopicInfo> for ForumTopicInfo {
    fn from(val: crate::types::ForumTopicInfo) -> Self {
        Self::ForumTopicInfo(Box::new(val))
    }
}

/// TDLib `ForumTopic` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ForumTopic {
    /// Describes a forum topic
    #[serde(rename(serialize = "forumTopic", deserialize = "forumTopic"))]
    ForumTopic(Box<crate::types::ForumTopic>),
}

impl ForumTopic {
    /// Convenience constructor to create a [`ForumTopic::ForumTopic`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn forum_topic(val: crate::types::ForumTopic) -> Self {
        Self::ForumTopic(Box::new(val))
    }

}

/// Converts a [`crate::types::ForumTopic`] into [`ForumTopic`].
impl From<crate::types::ForumTopic> for ForumTopic {
    fn from(val: crate::types::ForumTopic) -> Self {
        Self::ForumTopic(Box::new(val))
    }
}

/// TDLib `ForumTopics` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ForumTopics {
    /// Describes a list of forum topics
    #[serde(rename(serialize = "forumTopics", deserialize = "forumTopics"))]
    ForumTopics(Box<crate::types::ForumTopics>),
}

impl ForumTopics {
    /// Convenience constructor to create a [`ForumTopics::ForumTopics`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn forum_topics(val: crate::types::ForumTopics) -> Self {
        Self::ForumTopics(Box::new(val))
    }

}

/// Converts a [`crate::types::ForumTopics`] into [`ForumTopics`].
impl From<crate::types::ForumTopics> for ForumTopics {
    fn from(val: crate::types::ForumTopics) -> Self {
        Self::ForumTopics(Box::new(val))
    }
}

