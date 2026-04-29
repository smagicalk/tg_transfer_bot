#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A new message was received; can also be an outgoing message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewMessage {
    /// The new message
    pub message: crate::types::Message,
}
