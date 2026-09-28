//!
//! TDLib `notification` domain enums.
//!
//! Types, enums, and functions for push notifications and notification settings.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

/// Describes the types of chats to which notification settings are relevant
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

/// TDLib `ChatNotificationSettings` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatNotificationSettings {
    /// Contains information about notification settings for a chat or a forum topic
    #[serde(rename(serialize = "chatNotificationSettings", deserialize = "chatNotificationSettings"))]
    ChatNotificationSettings(Box<crate::types::ChatNotificationSettings>),
}

impl ChatNotificationSettings {
    /// Convenience constructor to create a [`ChatNotificationSettings::ChatNotificationSettings`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_notification_settings(val: crate::types::ChatNotificationSettings) -> Self {
        Self::ChatNotificationSettings(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatNotificationSettings`] into [`ChatNotificationSettings`].
impl From<crate::types::ChatNotificationSettings> for ChatNotificationSettings {
    fn from(val: crate::types::ChatNotificationSettings) -> Self {
        Self::ChatNotificationSettings(Box::new(val))
    }
}

/// TDLib `ScopeNotificationSettings` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ScopeNotificationSettings {
    /// Contains information about notification settings for several chats
    #[serde(rename(serialize = "scopeNotificationSettings", deserialize = "scopeNotificationSettings"))]
    ScopeNotificationSettings(Box<crate::types::ScopeNotificationSettings>),
}

impl ScopeNotificationSettings {
    /// Convenience constructor to create a [`ScopeNotificationSettings::ScopeNotificationSettings`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn scope_notification_settings(val: crate::types::ScopeNotificationSettings) -> Self {
        Self::ScopeNotificationSettings(Box::new(val))
    }

}

/// Converts a [`crate::types::ScopeNotificationSettings`] into [`ScopeNotificationSettings`].
impl From<crate::types::ScopeNotificationSettings> for ScopeNotificationSettings {
    fn from(val: crate::types::ScopeNotificationSettings) -> Self {
        Self::ScopeNotificationSettings(Box::new(val))
    }
}

/// Describes sources of reactions for which notifications will be shown
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ReactionNotificationSource {
    /// Notifications for reactions are disabled
    #[serde(rename(serialize = "reactionNotificationSourceNone", deserialize = "reactionNotificationSourceNone"))]
    None,
    /// Notifications for reactions are shown only for reactions from contacts
    #[serde(rename(serialize = "reactionNotificationSourceContacts", deserialize = "reactionNotificationSourceContacts"))]
    Contacts,
    /// Notifications for reactions are shown for all reactions
    #[serde(rename(serialize = "reactionNotificationSourceAll", deserialize = "reactionNotificationSourceAll"))]
    All,
}

/// TDLib `ReactionNotificationSettings` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ReactionNotificationSettings {
    /// Contains information about notification settings for reactions and poll votes
    #[serde(rename(serialize = "reactionNotificationSettings", deserialize = "reactionNotificationSettings"))]
    ReactionNotificationSettings(Box<crate::types::ReactionNotificationSettings>),
}

impl ReactionNotificationSettings {
    /// Convenience constructor to create a [`ReactionNotificationSettings::ReactionNotificationSettings`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn reaction_notification_settings(val: crate::types::ReactionNotificationSettings) -> Self {
        Self::ReactionNotificationSettings(Box::new(val))
    }

}

/// Converts a [`crate::types::ReactionNotificationSettings`] into [`ReactionNotificationSettings`].
impl From<crate::types::ReactionNotificationSettings> for ReactionNotificationSettings {
    fn from(val: crate::types::ReactionNotificationSettings) -> Self {
        Self::ReactionNotificationSettings(Box::new(val))
    }
}

/// Contains detailed information about a notification
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum NotificationType {
    /// New message was received
    #[serde(rename(serialize = "notificationTypeNewMessage", deserialize = "notificationTypeNewMessage"))]
    NewMessage(Box<crate::types::NotificationTypeNewMessage>),
    /// New secret chat was created
    #[serde(rename(serialize = "notificationTypeNewSecretChat", deserialize = "notificationTypeNewSecretChat"))]
    NewSecretChat,
    /// New call was received
    #[serde(rename(serialize = "notificationTypeNewCall", deserialize = "notificationTypeNewCall"))]
    NewCall(Box<crate::types::NotificationTypeNewCall>),
    /// New message was received through a push notification
    #[serde(rename(serialize = "notificationTypeNewPushMessage", deserialize = "notificationTypeNewPushMessage"))]
    NewPushMessage(Box<crate::types::NotificationTypeNewPushMessage>),
}

impl NotificationType {
    /// Convenience constructor to create a [`NotificationType::NewMessage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn new_message(val: crate::types::NotificationTypeNewMessage) -> Self {
        Self::NewMessage(Box::new(val))
    }

    /// Convenience constructor to create a [`NotificationType::NewCall`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn new_call(val: crate::types::NotificationTypeNewCall) -> Self {
        Self::NewCall(Box::new(val))
    }

    /// Convenience constructor to create a [`NotificationType::NewPushMessage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn new_push_message(val: crate::types::NotificationTypeNewPushMessage) -> Self {
        Self::NewPushMessage(Box::new(val))
    }

}

/// Converts a [`crate::types::NotificationTypeNewMessage`] into [`NotificationType`].
impl From<crate::types::NotificationTypeNewMessage> for NotificationType {
    fn from(val: crate::types::NotificationTypeNewMessage) -> Self {
        Self::NewMessage(Box::new(val))
    }
}

/// Converts a [`crate::types::NotificationTypeNewCall`] into [`NotificationType`].
impl From<crate::types::NotificationTypeNewCall> for NotificationType {
    fn from(val: crate::types::NotificationTypeNewCall) -> Self {
        Self::NewCall(Box::new(val))
    }
}

/// Converts a [`crate::types::NotificationTypeNewPushMessage`] into [`NotificationType`].
impl From<crate::types::NotificationTypeNewPushMessage> for NotificationType {
    fn from(val: crate::types::NotificationTypeNewPushMessage) -> Self {
        Self::NewPushMessage(Box::new(val))
    }
}

/// Describes the type of notifications in a notification group
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum NotificationGroupType {
    /// A group containing notifications of type notificationTypeNewMessage and notificationTypeNewPushMessage with ordinary unread messages
    #[serde(rename(serialize = "notificationGroupTypeMessages", deserialize = "notificationGroupTypeMessages"))]
    Messages,
    /// A group containing notifications of type notificationTypeNewMessage and notificationTypeNewPushMessage with unread mentions of the current user, replies to their messages, or a pinned message
    #[serde(rename(serialize = "notificationGroupTypeMentions", deserialize = "notificationGroupTypeMentions"))]
    Mentions,
    /// A group containing a notification of type notificationTypeNewSecretChat
    #[serde(rename(serialize = "notificationGroupTypeSecretChat", deserialize = "notificationGroupTypeSecretChat"))]
    SecretChat,
    /// A group containing notifications of type notificationTypeNewCall
    #[serde(rename(serialize = "notificationGroupTypeCalls", deserialize = "notificationGroupTypeCalls"))]
    Calls,
}

/// TDLib `NotificationSound` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum NotificationSound {
    /// Describes a notification sound in MP3 format
    #[serde(rename(serialize = "notificationSound", deserialize = "notificationSound"))]
    NotificationSound(Box<crate::types::NotificationSound>),
}

impl NotificationSound {
    /// Convenience constructor to create a [`NotificationSound::NotificationSound`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn notification_sound(val: crate::types::NotificationSound) -> Self {
        Self::NotificationSound(Box::new(val))
    }

}

/// Converts a [`crate::types::NotificationSound`] into [`NotificationSound`].
impl From<crate::types::NotificationSound> for NotificationSound {
    fn from(val: crate::types::NotificationSound) -> Self {
        Self::NotificationSound(Box::new(val))
    }
}

/// TDLib `NotificationSounds` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum NotificationSounds {
    /// Contains a list of notification sounds
    #[serde(rename(serialize = "notificationSounds", deserialize = "notificationSounds"))]
    NotificationSounds(Box<crate::types::NotificationSounds>),
}

impl NotificationSounds {
    /// Convenience constructor to create a [`NotificationSounds::NotificationSounds`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn notification_sounds(val: crate::types::NotificationSounds) -> Self {
        Self::NotificationSounds(Box::new(val))
    }

}

/// Converts a [`crate::types::NotificationSounds`] into [`NotificationSounds`].
impl From<crate::types::NotificationSounds> for NotificationSounds {
    fn from(val: crate::types::NotificationSounds) -> Self {
        Self::NotificationSounds(Box::new(val))
    }
}

/// TDLib `Notification` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Notification {
    /// Contains information about a notification
    #[serde(rename(serialize = "notification", deserialize = "notification"))]
    Notification(Box<crate::types::Notification>),
}

impl Notification {
    /// Convenience constructor to create a [`Notification::Notification`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn notification(val: crate::types::Notification) -> Self {
        Self::Notification(Box::new(val))
    }

}

/// Converts a [`crate::types::Notification`] into [`Notification`].
impl From<crate::types::Notification> for Notification {
    fn from(val: crate::types::Notification) -> Self {
        Self::Notification(Box::new(val))
    }
}

/// TDLib `NotificationGroup` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum NotificationGroup {
    /// Describes a group of notifications
    #[serde(rename(serialize = "notificationGroup", deserialize = "notificationGroup"))]
    NotificationGroup(Box<crate::types::NotificationGroup>),
}

impl NotificationGroup {
    /// Convenience constructor to create a [`NotificationGroup::NotificationGroup`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn notification_group(val: crate::types::NotificationGroup) -> Self {
        Self::NotificationGroup(Box::new(val))
    }

}

/// Converts a [`crate::types::NotificationGroup`] into [`NotificationGroup`].
impl From<crate::types::NotificationGroup> for NotificationGroup {
    fn from(val: crate::types::NotificationGroup) -> Self {
        Self::NotificationGroup(Box::new(val))
    }
}

