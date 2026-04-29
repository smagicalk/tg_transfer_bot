#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A request to send a message has reached the Telegram server. This doesn't mean that the message will be sent successfully.
/// This update is sent only if the option "use_quick_ack" is set to true. This update may be sent multiple times for the same message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageSendAcknowledged {
    /// The chat identifier of the sent message
    pub chat_id: i64,
    /// A temporary message identifier
    pub message_id: i64,
}
