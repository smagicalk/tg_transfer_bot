#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The user is under certain restrictions in the chat. Not supported in basic groups and channels
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatMemberStatusRestricted {
    /// True, if the user is a member of the chat
    pub is_member: bool,
    /// Point in time (Unix timestamp) when restrictions will be lifted from the user; 0 if never. If the user is restricted for more than 366 days or for less than 30 seconds from the current time, the user is considered to be restricted forever
    pub restricted_until_date: i32,
    /// User permissions in the chat
    pub permissions: crate::types::ChatPermissions,
}
