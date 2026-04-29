#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Notification settings for a chat were changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatNotificationSettings {
    /// Chat identifier
    pub chat_id: i64,
    /// The new notification settings
    pub notification_settings: crate::types::ChatNotificationSettings,
}
