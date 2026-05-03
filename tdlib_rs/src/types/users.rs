#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a list of users
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Users {
    /// Approximate total number of users found
    pub total_count: i32,
    /// A list of user identifiers
    pub user_ids: Vec<i64>,
}
