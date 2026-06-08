#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatNotificationSettings {
    /// Contains information about notification settings for a chat or a forum topic
    #[serde(rename(
        serialize = "chatNotificationSettings",
        deserialize = "chatNotificationSettings"
    ))]
    ChatNotificationSettings(crate::types::ChatNotificationSettings),
}
