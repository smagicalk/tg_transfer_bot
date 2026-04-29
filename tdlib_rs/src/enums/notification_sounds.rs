#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum NotificationSounds {
    /// Contains a list of notification sounds
    #[serde(rename(serialize = "notificationSounds", deserialize = "notificationSounds"))]
    NotificationSounds(crate::types::NotificationSounds),
}
