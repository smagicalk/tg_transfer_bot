#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a group of notifications
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NotificationGroup {
    /// Unique persistent auto-incremented from 1 identifier of the notification group
    pub id: i32,
    /// Type of the group
    pub r#type: crate::enums::NotificationGroupType,
    /// Identifier of a chat to which all notifications in the group belong
    pub chat_id: i64,
    /// Total number of active notifications in the group
    pub total_count: i32,
    /// The list of active notifications
    pub notifications: Vec<crate::types::Notification>,
}
