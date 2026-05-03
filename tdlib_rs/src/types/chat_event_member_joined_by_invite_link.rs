#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A new member joined the chat via an invite link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventMemberJoinedByInviteLink {
    /// Invite link used to join the chat
    pub invite_link: crate::types::ChatInviteLink,
    /// True, if the user has joined the chat using an invite link for a chat folder
    pub via_chat_folder_invite_link: bool,
}
