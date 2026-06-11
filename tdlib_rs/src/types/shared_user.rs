#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about a user shared with a bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SharedUser {
    /// User identifier
    pub user_id: i64,
    /// First name of the user; for bots only
    pub first_name: String,
    /// Last name of the user; for bots only
    pub last_name: String,
    /// Username of the user; for bots only
    pub username: String,
    /// Profile photo of the user; for bots only; may be null
    pub photo: Option<crate::types::Photo>,
}
