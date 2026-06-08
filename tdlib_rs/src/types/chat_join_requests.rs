#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of requests to join a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatJoinRequests {
    /// Approximate total number of requests found
    pub total_count: i32,
    /// List of the requests
    pub requests: Vec<crate::types::ChatJoinRequest>,
}
