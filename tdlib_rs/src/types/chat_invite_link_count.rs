#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a chat administrator with a number of active and revoked chat invite links
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatInviteLinkCount {
    /// Administrator's user identifier
    pub user_id: i64,
    /// Number of active invite links
    pub invite_link_count: i32,
    /// Number of revoked invite links
    pub revoked_invite_link_count: i32,
}
