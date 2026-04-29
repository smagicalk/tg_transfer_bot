#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a chat invite link. Call checkChatInviteLink with the given invite link to process the link.
/// If the link is valid and the user wants to join the chat, then call joinChatByInviteLink
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeChatInvite {
    /// Internal representation of the invite link
    pub invite_link: String,
}
