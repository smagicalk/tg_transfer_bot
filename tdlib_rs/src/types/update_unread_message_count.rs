#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Number of unread messages in a chat list has changed. This update is sent only if the message database is used
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateUnreadMessageCount {
    /// The chat list with changed number of unread messages
    pub chat_list: crate::enums::ChatList,
    /// Total number of unread messages
    pub unread_count: i32,
    /// Total number of unread messages in unmuted chats
    pub unread_unmuted_count: i32,
}
