#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The position of a chat in a chat list has changed. An updateChatLastMessage or updateChatDraftMessage update might be sent instead of the update
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatPosition {
    /// Chat identifier
    pub chat_id: i64,
    /// New chat position. If new order is 0, then the chat needs to be removed from the list
    pub position: crate::types::ChatPosition,
}
