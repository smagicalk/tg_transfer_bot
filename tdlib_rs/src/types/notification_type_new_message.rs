#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// New message was received
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NotificationTypeNewMessage {
    /// The message
    pub message: crate::types::Message,
    /// True, if message content must be displayed in notifications
    pub show_preview: bool,
}
