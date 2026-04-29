#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a folder for user chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatFolder {
    /// The name of the folder
    pub name: crate::types::ChatFolderName,
    /// The chosen icon for the chat folder; may be null. If null, use getChatFolderDefaultIconName to get default icon name for the folder
    pub icon: Option<crate::types::ChatFolderIcon>,
    /// The identifier of the chosen color for the chat folder icon; from -1 to 6. If -1, then color is disabled. Can't be changed if folder tags are disabled or the current user doesn't have Telegram Premium subscription
    pub color_id: i32,
    /// True, if at least one link has been created for the folder
    pub is_shareable: bool,
    /// The chat identifiers of pinned chats in the folder. There can be up to getOption("chat_folder_chosen_chat_count_max") pinned and always included non-secret chats and the same number of secret chats, but the limit can be increased with Telegram Premium
    pub pinned_chat_ids: Vec<i64>,
    /// The chat identifiers of always included chats in the folder. There can be up to getOption("chat_folder_chosen_chat_count_max") pinned and always included non-secret chats and the same number of secret chats, but the limit can be increased with Telegram Premium
    pub included_chat_ids: Vec<i64>,
    /// The chat identifiers of always excluded chats in the folder. There can be up to getOption("chat_folder_chosen_chat_count_max") always excluded non-secret chats and the same number of secret chats, but the limit can be increased with Telegram Premium
    pub excluded_chat_ids: Vec<i64>,
    /// True, if muted chats need to be excluded
    pub exclude_muted: bool,
    /// True, if read chats need to be excluded
    pub exclude_read: bool,
    /// True, if archived chats need to be excluded
    pub exclude_archived: bool,
    /// True, if contacts need to be included
    pub include_contacts: bool,
    /// True, if non-contact users need to be included
    pub include_non_contacts: bool,
    /// True, if bots need to be included
    pub include_bots: bool,
    /// True, if basic groups and supergroups need to be included
    pub include_groups: bool,
    /// True, if channels need to be included
    pub include_channels: bool,
}
