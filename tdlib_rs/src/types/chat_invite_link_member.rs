#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a chat member joined a chat via an invite link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatInviteLinkMember {
    /// User identifier
    pub user_id: i64,
    /// Point in time (Unix timestamp) when the user joined the chat
    pub joined_chat_date: i32,
    /// True, if the user has joined the chat using an invite link for a chat folder
    pub via_chat_folder_invite_link: bool,
    /// User identifier of the chat administrator, approved user join request
    pub approver_user_id: i64,
}
