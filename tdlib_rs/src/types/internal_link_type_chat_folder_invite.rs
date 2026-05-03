#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is an invite link to a chat folder. Call checkChatFolderInviteLink with the given invite link to process the link.
/// If the link is valid and the user wants to join the chat folder, then call addChatFolderByInviteLink
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeChatFolderInvite {
    /// Internal representation of the invite link
    pub invite_link: String,
}
