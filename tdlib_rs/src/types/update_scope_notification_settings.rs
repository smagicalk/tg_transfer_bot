#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Notification settings for some type of chats were updated
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateScopeNotificationSettings {
    /// Types of chats for which notification settings were updated
    pub scope: crate::enums::NotificationSettingsScope,
    /// The new notification settings
    pub notification_settings: crate::types::ScopeNotificationSettings,
}
