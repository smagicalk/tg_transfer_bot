#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A message has been successfully sent
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageSendSucceeded {
    /// The sent message. Almost any field of the new message can be different from the corresponding field of the original message.
    /// For example, the field scheduling_state may change, making the message scheduled, or non-scheduled
    pub message: crate::types::Message,
    /// The previous temporary message identifier
    pub old_message_id: i64,
}
