#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The user is the owner of the chat and has all the administrator privileges
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatMemberStatusCreator {
    /// True, if the creator isn't shown in the chat member list and sends messages anonymously; applicable to supergroups only
    pub is_anonymous: bool,
    /// True, if the user is a member of the chat
    pub is_member: bool,
}
