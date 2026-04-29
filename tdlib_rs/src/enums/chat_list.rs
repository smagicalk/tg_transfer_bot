#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatList {
    /// A main list of chats
    #[serde(rename(serialize = "chatListMain", deserialize = "chatListMain"))]
    Main,
    /// A list of chats usually located at the top of the main chat list. Unmuted chats are automatically moved from the Archive to the Main chat list when a new message arrives
    #[serde(rename(serialize = "chatListArchive", deserialize = "chatListArchive"))]
    Archive,
    /// A list of chats added to a chat folder
    #[serde(rename(serialize = "chatListFolder", deserialize = "chatListFolder"))]
    Folder(crate::types::ChatListFolder),
}
