#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The chat pending join requests were changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatPendingJoinRequests {
    /// Chat identifier
    pub chat_id: i64,
    /// The new data about pending join requests; may be null
    pub pending_join_requests: Option<crate::types::ChatJoinRequestsInfo>,
}
