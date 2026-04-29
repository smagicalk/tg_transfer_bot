#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The information about interactions with a message has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageInteractionInfo {
    /// Chat identifier
    pub chat_id: i64,
    /// Message identifier
    pub message_id: i64,
    /// New information about interactions with the message; may be null
    pub interaction_info: Option<crate::types::MessageInteractionInfo>,
}
