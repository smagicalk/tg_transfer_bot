#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A chat invite link was edited
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventInviteLinkEdited {
    /// Previous information about the invite link
    pub old_invite_link: crate::types::ChatInviteLink,
    /// New information about the invite link
    pub new_invite_link: crate::types::ChatInviteLink,
}
