#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A new message was received in a group call
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewGroupCallMessage {
    /// Identifier of the group call
    pub group_call_id: i32,
    /// The message
    pub message: crate::types::GroupCallMessage,
}
