#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a message from a business account as received by a bot
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BusinessMessage {
    /// The message
    pub message: crate::types::Message,
    /// Message that is replied by the message in the same chat; may be null if none
    pub reply_to_message: Option<crate::types::Message>,
}
