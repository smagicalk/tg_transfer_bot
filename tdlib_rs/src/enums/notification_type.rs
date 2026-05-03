#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum NotificationType {
    /// New message was received
    #[serde(rename(
        serialize = "notificationTypeNewMessage",
        deserialize = "notificationTypeNewMessage"
    ))]
    NewMessage(crate::types::NotificationTypeNewMessage),
    /// New secret chat was created
    #[serde(rename(
        serialize = "notificationTypeNewSecretChat",
        deserialize = "notificationTypeNewSecretChat"
    ))]
    NewSecretChat,
    /// New call was received
    #[serde(rename(
        serialize = "notificationTypeNewCall",
        deserialize = "notificationTypeNewCall"
    ))]
    NewCall(crate::types::NotificationTypeNewCall),
    /// New message was received through a push notification
    #[serde(rename(
        serialize = "notificationTypeNewPushMessage",
        deserialize = "notificationTypeNewPushMessage"
    ))]
    NewPushMessage(crate::types::NotificationTypeNewPushMessage),
}
