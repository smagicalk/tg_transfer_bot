#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A call from a message of the type messageCall with non-zero messageCall.unique_id
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputCallFromMessage {
    /// Chat identifier of the message
    pub chat_id: i64,
    /// Message identifier
    pub message_id: i64,
}
