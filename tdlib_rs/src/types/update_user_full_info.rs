#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Some data in userFullInfo has been changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateUserFullInfo {
    /// User identifier
    pub user_id: i64,
    /// New full information about the user
    pub user_full_info: crate::types::UserFullInfo,
}
