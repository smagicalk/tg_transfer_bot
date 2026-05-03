#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A chat member was restricted/unrestricted or banned/unbanned, or the list of their restrictions has changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventMemberRestricted {
    /// Affected chat member identifier
    pub member_id: crate::enums::MessageSender,
    /// Previous status of the chat member
    pub old_status: crate::enums::ChatMemberStatus,
    /// New status of the chat member
    pub new_status: crate::enums::ChatMemberStatus,
}
