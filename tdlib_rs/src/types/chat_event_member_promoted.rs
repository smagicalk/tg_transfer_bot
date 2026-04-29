#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A chat member has gained/lost administrator status, or the list of their administrator privileges has changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventMemberPromoted {
    /// Affected chat member user identifier
    pub user_id: i64,
    /// Previous status of the chat member
    pub old_status: crate::enums::ChatMemberStatus,
    /// New status of the chat member
    pub new_status: crate::enums::ChatMemberStatus,
}
