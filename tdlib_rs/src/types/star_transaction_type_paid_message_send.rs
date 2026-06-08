#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The transaction is a sending of a paid message; relevant for regular users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypePaidMessageSend {
    /// Identifier of the chat that received the payment
    pub chat_id: i64,
    /// Number of sent paid messages
    pub message_count: i32,
}
