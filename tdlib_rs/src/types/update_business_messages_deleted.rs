#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Messages in a business account were deleted; for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateBusinessMessagesDeleted {
    /// Unique identifier of the business connection
    pub connection_id: String,
    /// Identifier of a chat in the business account in which messages were deleted
    pub chat_id: i64,
    /// Unique message identifiers of the deleted messages
    pub message_ids: Vec<i64>,
}
