#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A message with a live location was viewed. When the update is received, the application is expected to update the live location
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageLiveLocationViewed {
    /// Identifier of the chat with the live location message
    pub chat_id: i64,
    /// Identifier of the message with live location
    pub message_id: i64,
}
