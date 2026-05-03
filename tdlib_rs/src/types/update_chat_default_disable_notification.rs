#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The value of the default disable_notification parameter, used when a message is sent to the chat, was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatDefaultDisableNotification {
    /// Chat identifier
    pub chat_id: i64,
    /// The new default_disable_notification value
    pub default_disable_notification: bool,
}
