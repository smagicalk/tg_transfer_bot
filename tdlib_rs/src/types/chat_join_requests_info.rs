#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about pending join requests for a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatJoinRequestsInfo {
    /// Total number of pending join requests
    pub total_count: i32,
    /// Identifiers of at most 3 users sent the newest pending join requests
    pub user_ids: Vec<i64>,
}
