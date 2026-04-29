#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a user who sent a join request and waits for administrator approval
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatJoinRequest {
    /// User identifier
    pub user_id: i64,
    /// Point in time (Unix timestamp) when the user sent the join request
    pub date: i32,
    /// A short bio of the user
    pub bio: String,
}
