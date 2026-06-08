#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about a chat administrator
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatAdministrator {
    /// User identifier of the administrator
    pub user_id: i64,
    /// Custom title of the administrator
    pub custom_title: String,
    /// True, if the user is the owner of the chat
    pub is_owner: bool,
}
