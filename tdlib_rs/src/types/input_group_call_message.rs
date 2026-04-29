#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The group call is accessible through a message of the type messageGroupCall
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputGroupCallMessage {
    /// Identifier of the chat with the message
    pub chat_id: i64,
    /// Identifier of the message of the type messageGroupCall
    pub message_id: i64,
}
