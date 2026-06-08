#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// New chat members were added
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatAddMembers {
    /// User identifiers of the new members
    pub member_user_ids: Vec<i64>,
}
