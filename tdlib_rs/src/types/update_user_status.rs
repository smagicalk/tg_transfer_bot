#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The user went online or offline
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateUserStatus {
    /// User identifier
    pub user_id: i64,
    /// New status of the user
    pub status: crate::enums::UserStatus,
}
