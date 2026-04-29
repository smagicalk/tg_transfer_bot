#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents an inline message that can be sent via the bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PreparedInlineMessageId {
    /// Unique identifier for the message
    pub id: String,
    /// Point in time (Unix timestamp) when the message can't be used anymore
    pub expiration_date: i32,
}
