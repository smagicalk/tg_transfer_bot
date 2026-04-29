#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A new member was accepted to the chat by an administrator
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventMemberJoinedByRequest {
    /// User identifier of the chat administrator, approved user join request
    pub approver_user_id: i64,
    /// Invite link used to join the chat; may be null
    pub invite_link: Option<crate::types::ChatInviteLink>,
}
