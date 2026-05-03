#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A revoked chat invite link was deleted
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventInviteLinkDeleted {
    /// The invite link
    pub invite_link: crate::types::ChatInviteLink,
}
