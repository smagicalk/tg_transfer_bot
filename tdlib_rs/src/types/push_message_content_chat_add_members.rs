#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// New chat members were invited to a group
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentChatAddMembers {
    /// Name of the added member
    pub member_name: String,
    /// True, if the current user was added to the group
    pub is_current_user: bool,
    /// True, if the user has returned to the group themselves
    pub is_returned: bool,
}
