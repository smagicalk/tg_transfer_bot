#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A chat member was deleted
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentChatDeleteMember {
    /// Name of the deleted member
    pub member_name: String,
    /// True, if the current user was deleted from the group
    pub is_current_user: bool,
    /// True, if the user has left the group themselves
    pub is_left: bool,
}
