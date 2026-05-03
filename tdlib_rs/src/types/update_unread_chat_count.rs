#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Number of unread chats, i.e. with unread messages or marked as unread, has changed. This update is sent only if the message database is used
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateUnreadChatCount {
    /// The chat list with changed number of unread messages
    pub chat_list: crate::enums::ChatList,
    /// Approximate total number of chats in the chat list
    pub total_count: i32,
    /// Total number of unread chats
    pub unread_count: i32,
    /// Total number of unread unmuted chats
    pub unread_unmuted_count: i32,
    /// Total number of chats marked as unread
    pub marked_as_unread_count: i32,
    /// Total number of unmuted chats marked as unread
    pub marked_as_unread_unmuted_count: i32,
}
