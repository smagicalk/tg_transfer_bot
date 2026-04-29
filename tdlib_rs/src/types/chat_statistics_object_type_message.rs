#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a message sent in the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatStatisticsObjectTypeMessage {
    /// Message identifier
    pub message_id: i64,
}
