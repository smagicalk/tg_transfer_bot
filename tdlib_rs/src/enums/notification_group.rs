#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum NotificationGroup {
    /// Describes a group of notifications
    #[serde(rename(serialize = "notificationGroup", deserialize = "notificationGroup"))]
    NotificationGroup(crate::types::NotificationGroup),
}
