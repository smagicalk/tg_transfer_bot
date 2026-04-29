#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A group call message failed to send
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateGroupCallMessageSendFailed {
    /// Identifier of the group call
    pub group_call_id: i32,
    /// Message identifier
    pub message_id: i32,
    /// The cause of the message sending failure
    pub error: crate::types::Error,
}
