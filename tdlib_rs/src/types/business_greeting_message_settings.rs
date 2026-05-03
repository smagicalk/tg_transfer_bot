#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes settings for greeting messages that are automatically sent by a Telegram Business account as response to incoming messages in an inactive private chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessGreetingMessageSettings {
    /// Unique quick reply shortcut identifier for the greeting messages
    pub shortcut_id: i32,
    /// Chosen recipients of the greeting messages
    pub recipients: crate::types::BusinessRecipients,
    /// The number of days after which a chat will be considered as inactive; currently, must be on of 7, 14, 21, or 28
    pub inactivity_days: i32,
}
