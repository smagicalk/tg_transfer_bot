#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains settings for automatic moving of chats to and from the Archive chat lists
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ArchiveChatListSettings {
    /// True, if new chats from non-contacts will be automatically archived and muted. Can be set to true only if the option "can_archive_and_mute_new_chats_from_unknown_users" is true
    pub archive_and_mute_new_chats_from_unknown_users: bool,
    /// True, if unmuted chats will be kept in the Archive chat list when they get a new message
    pub keep_unmuted_chats_archived: bool,
    /// True, if unmuted chats, that are always included or pinned in a folder, will be kept in the Archive chat list when they get a new message. Ignored if keep_unmuted_chats_archived == true
    pub keep_chats_from_folders_archived: bool,
}
