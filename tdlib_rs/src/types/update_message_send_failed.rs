#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A message failed to send. Be aware that some messages being sent can be irrecoverably deleted, in which case updateDeleteMessages will be received instead of this update
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageSendFailed {
    /// The failed to send message
    pub message: crate::types::Message,
    /// The previous temporary message identifier
    pub old_message_id: i64,
    /// The cause of the message sending failure
    pub error: crate::types::Error,
}
