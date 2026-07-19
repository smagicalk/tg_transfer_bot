#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// User rights changed in a chat; for bots only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatMember {
    /// Chat identifier
    pub chat_id: i64,
    /// Identifier of the user, changing the rights
    pub actor_user_id: i64,
    /// Point in time (Unix timestamp) when the user rights were changed
    pub date: i32,
    /// If user has joined the chat using an invite link, the invite link; may be null
    pub invite_link: Option<crate::types::ChatInviteLink>,
    /// True, if the user has joined the chat after sending a join request and being approved by an administrator
    pub via_join_request: bool,
    /// True, if the user has joined the chat using an invite link for a chat folder
    pub via_chat_folder_invite_link: bool,
    /// Previous chat member
    pub old_chat_member: crate::types::ChatMember,
    /// New chat member
    pub new_chat_member: crate::types::ChatMember,
}
