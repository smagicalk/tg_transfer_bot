#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about a user who has failed to be added to a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FailedToAddMember {
    /// User identifier
    pub user_id: i64,
    /// True, if subscription to Telegram Premium would have allowed to add the user to the chat
    pub premium_would_allow_invite: bool,
    /// True, if subscription to Telegram Premium is required to send the user chat invite link
    pub premium_required_to_send_messages: bool,
}
