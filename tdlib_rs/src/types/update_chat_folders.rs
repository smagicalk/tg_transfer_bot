#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The list of chat folders or a chat folder has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatFolders {
    /// The new list of chat folders
    pub chat_folders: Vec<crate::types::ChatFolderInfo>,
    /// Position of the main chat list among chat folders, 0-based
    pub main_chat_list_position: i32,
    /// True, if folder tags are enabled
    pub are_tags_enabled: bool,
}
