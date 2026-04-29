#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum NotificationSound {
    /// Describes a notification sound in MP3 format
    #[serde(rename(serialize = "notificationSound", deserialize = "notificationSound"))]
    NotificationSound(crate::types::NotificationSound),
}
