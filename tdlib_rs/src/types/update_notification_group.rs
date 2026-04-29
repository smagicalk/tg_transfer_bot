#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A list of active notifications in a notification group has changed
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateNotificationGroup {
    /// Unique notification group identifier
    pub notification_group_id: i32,
    /// New type of the notification group
    pub r#type: crate::enums::NotificationGroupType,
    /// Identifier of a chat to which all notifications in the group belong
    pub chat_id: i64,
    /// Chat identifier, which notification settings must be applied to the added notifications
    pub notification_settings_chat_id: i64,
    /// Identifier of the notification sound to be played; 0 if sound is disabled
    #[serde_as(as = "DisplayFromStr")]
    pub notification_sound_id: i64,
    /// Total number of unread notifications in the group, can be bigger than number of active notifications
    pub total_count: i32,
    /// List of added group notifications, sorted by notification identifier
    pub added_notifications: Vec<crate::types::Notification>,
    /// Identifiers of removed group notifications, sorted by notification identifier
    pub removed_notification_ids: Vec<i32>,
}
