#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about an invite link to a chat folder
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatFolderInviteLinkInfo {
    /// Basic information about the chat folder; chat folder identifier will be 0 if the user didn't have the chat folder yet
    pub chat_folder_info: crate::types::ChatFolderInfo,
    /// Identifiers of the chats from the link, which aren't added to the folder yet
    pub missing_chat_ids: Vec<i64>,
    /// Identifiers of the chats from the link, which are added to the folder already
    pub added_chat_ids: Vec<i64>,
}
