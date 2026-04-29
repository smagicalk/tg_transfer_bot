#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Some group call messages were deleted
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateGroupCallMessagesDeleted {
    /// Identifier of the group call
    pub group_call_id: i32,
    /// Identifiers of the deleted messages
    pub message_ids: Vec<i32>,
}
