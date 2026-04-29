#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The user is a member of the chat and has some additional privileges. In basic groups, administrators can edit and delete messages sent by others, add new members, ban unprivileged members, and manage video chats.
/// In supergroups and channels, there are more detailed options for administrator privileges
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatMemberStatusAdministrator {
    /// True, if the current user can edit the administrator privileges for the called user
    pub can_be_edited: bool,
    /// Rights of the administrator
    pub rights: crate::types::ChatAdministratorRights,
}
