#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A chat member tag has been changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventMemberTagChanged {
    /// Affected chat member user identifier
    pub user_id: i64,
    /// Previous tag of the chat member
    pub old_tag: String,
    /// New tag of the chat member
    pub new_tag: String,
}
