#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains statistics about number of new members invited by a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatStatisticsInviterInfo {
    /// User identifier
    pub user_id: i64,
    /// Number of new members invited by the user
    pub added_member_count: i32,
}
