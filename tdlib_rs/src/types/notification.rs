#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about a notification
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Notification {
    /// Unique persistent identifier of this notification
    pub id: i32,
    /// Notification date
    pub date: i32,
    /// True, if the notification was explicitly sent without sound
    pub is_silent: bool,
    /// Notification type
    pub r#type: crate::enums::NotificationType,
}
