#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The can_invite_users permission of a supergroup chat was toggled
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventInvitesToggled {
    /// New value of can_invite_users permission
    pub can_invite_users: bool,
}
