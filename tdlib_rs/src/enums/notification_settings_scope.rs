#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum NotificationSettingsScope {
    /// Notification settings applied to all private and secret chats when the corresponding chat setting has a default value
    #[serde(rename(serialize = "notificationSettingsScopePrivateChats", deserialize = "notificationSettingsScopePrivateChats"))]
    PrivateChats,
    /// Notification settings applied to all basic group and supergroup chats when the corresponding chat setting has a default value
    #[serde(rename(serialize = "notificationSettingsScopeGroupChats", deserialize = "notificationSettingsScopeGroupChats"))]
    GroupChats,
    /// Notification settings applied to all channel chats when the corresponding chat setting has a default value
    #[serde(rename(serialize = "notificationSettingsScopeChannelChats", deserialize = "notificationSettingsScopeChannelChats"))]
    ChannelChats,
}
