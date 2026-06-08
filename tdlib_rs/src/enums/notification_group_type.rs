#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum NotificationGroupType {
    /// A group containing notifications of type notificationTypeNewMessage and notificationTypeNewPushMessage with ordinary unread messages
    #[serde(rename(
        serialize = "notificationGroupTypeMessages",
        deserialize = "notificationGroupTypeMessages"
    ))]
    Messages,
    /// A group containing notifications of type notificationTypeNewMessage and notificationTypeNewPushMessage with unread mentions of the current user, replies to their messages, or a pinned message
    #[serde(rename(
        serialize = "notificationGroupTypeMentions",
        deserialize = "notificationGroupTypeMentions"
    ))]
    Mentions,
    /// A group containing a notification of type notificationTypeNewSecretChat
    #[serde(rename(
        serialize = "notificationGroupTypeSecretChat",
        deserialize = "notificationGroupTypeSecretChat"
    ))]
    SecretChat,
    /// A group containing notifications of type notificationTypeNewCall
    #[serde(rename(
        serialize = "notificationGroupTypeCalls",
        deserialize = "notificationGroupTypeCalls"
    ))]
    Calls,
}
