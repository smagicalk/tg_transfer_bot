#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A new chat member was invited
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventMemberInvited {
    /// New member user identifier
    pub user_id: i64,
    /// New member status
    pub status: crate::enums::ChatMemberStatus,
}
