#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A user sent a join request to a chat; for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewChatJoinRequest {
    /// Chat identifier
    pub chat_id: i64,
    /// Join request
    pub request: crate::types::ChatJoinRequest,
    /// Chat identifier of the private chat with the user
    pub user_chat_id: i64,
    /// The invite link, which was used to send join request; may be null
    pub invite_link: Option<crate::types::ChatInviteLink>,
}
