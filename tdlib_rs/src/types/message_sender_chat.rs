#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The message was sent on behalf of a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSenderChat {
    /// Identifier of the chat that sent the message
    pub chat_id: i64,
}
