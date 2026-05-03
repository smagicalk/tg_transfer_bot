#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Outgoing messages were read
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatReadOutbox {
    /// Chat identifier
    pub chat_id: i64,
    /// Identifier of last read outgoing message
    pub last_read_outbox_message_id: i64,
}
