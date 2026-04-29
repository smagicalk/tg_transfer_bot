#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a chat folder invite link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatFolderInviteLink {
    /// The chat folder invite link
    pub invite_link: String,
    /// Name of the link
    pub name: String,
    /// Identifiers of chats, included in the link
    pub chat_ids: Vec<i64>,
}
