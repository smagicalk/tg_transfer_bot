//!
//! TDLib `notification` domain types.
//!
//! Types, enums, and functions for push notifications and notification settings.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The message is from a notification
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSourceNotification {
}

/// Notification settings applied to all private and secret chats when the corresponding chat setting has a default value
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NotificationSettingsScopePrivateChats {
}

/// Notification settings applied to all basic group and supergroup chats when the corresponding chat setting has a default value
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NotificationSettingsScopeGroupChats {
}

/// Notification settings applied to all channel chats when the corresponding chat setting has a default value
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NotificationSettingsScopeChannelChats {
}

/// Contains information about notification settings for a chat or a forum topic
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatNotificationSettings {
    /// If true, the value for the relevant type of chat or the forum chat is used instead of mute_for
    pub use_default_mute_for: bool,
    /// Time left before notifications will be unmuted, in seconds
    pub mute_for: i32,
    /// If true, the value for the relevant type of chat or the forum chat is used instead of sound_id
    pub use_default_sound: bool,
    /// Identifier of the notification sound to be played for messages; 0 if sound is disabled
    #[serde_as(as = "DisplayFromStr")]
    pub sound_id: i64,
    /// If true, the value for the relevant type of chat or the forum chat is used instead of show_preview
    pub use_default_show_preview: bool,
    /// True, if message content must be displayed in notifications
    pub show_preview: bool,
    /// If true, the value for the relevant type of chat is used instead of mute_stories
    pub use_default_mute_stories: bool,
    /// True, if story notifications are disabled for the chat
    pub mute_stories: bool,
    /// If true, the value for the relevant type of chat is used instead of story_sound_id
    pub use_default_story_sound: bool,
    /// Identifier of the notification sound to be played for stories; 0 if sound is disabled
    #[serde_as(as = "DisplayFromStr")]
    pub story_sound_id: i64,
    /// If true, the value for the relevant type of chat is used instead of show_story_poster
    pub use_default_show_story_poster: bool,
    /// True, if the chat that posted a story must be displayed in notifications
    pub show_story_poster: bool,
    /// If true, the value for the relevant type of chat or the forum chat is used instead of disable_pinned_message_notifications
    pub use_default_disable_pinned_message_notifications: bool,
    /// If true, notifications for incoming pinned messages will be created as for an ordinary unread message
    pub disable_pinned_message_notifications: bool,
    /// If true, the value for the relevant type of chat or the forum chat is used instead of disable_mention_notifications
    pub use_default_disable_mention_notifications: bool,
    /// If true, notifications for messages with mentions will be created as for an ordinary unread message
    pub disable_mention_notifications: bool,
}

/// Contains information about notification settings for several chats
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ScopeNotificationSettings {
    /// Time left before notifications will be unmuted, in seconds
    pub mute_for: i32,
    /// Identifier of the notification sound to be played; 0 if sound is disabled; pass -1 to use the app-dependent default sound
    #[serde_as(as = "DisplayFromStr")]
    pub sound_id: i64,
    /// True, if message content must be displayed in notifications
    pub show_preview: bool,
    /// If true, story notifications are received only for the first 5 chats from topChatCategoryUsers regardless of the value of mute_stories
    pub use_default_mute_stories: bool,
    /// True, if story notifications are disabled
    pub mute_stories: bool,
    /// Identifier of the notification sound to be played for stories; 0 if sound is disabled; pass -1 to use the app-dependent default sound
    #[serde_as(as = "DisplayFromStr")]
    pub story_sound_id: i64,
    /// True, if the chat that posted a story must be displayed in notifications
    pub show_story_poster: bool,
    /// True, if notifications for incoming pinned messages will be created as for an ordinary unread message
    pub disable_pinned_message_notifications: bool,
    /// True, if notifications for messages with mentions will be created as for an ordinary unread message
    pub disable_mention_notifications: bool,
}

/// Notifications for reactions are disabled
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReactionNotificationSourceNone {
}

/// Notifications for reactions are shown only for reactions from contacts
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReactionNotificationSourceContacts {
}

/// Notifications for reactions are shown for all reactions
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReactionNotificationSourceAll {
}

/// Contains information about notification settings for reactions and poll votes
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ReactionNotificationSettings {
    /// Source of message reactions for which notifications are shown
    pub message_reaction_source: crate::enums::ReactionNotificationSource,
    /// Source of story reactions for which notifications are shown
    pub story_reaction_source: crate::enums::ReactionNotificationSource,
    /// Source of poll votes for which notifications are shown
    pub poll_vote_source: crate::enums::ReactionNotificationSource,
    /// Identifier of the notification sound to be played; 0 if sound is disabled; pass -1 to use the app-dependent default sound
    #[serde_as(as = "DisplayFromStr")]
    pub sound_id: i64,
    /// True, if reaction sender and emoji must be displayed in notifications
    pub show_preview: bool,
}

/// New message was received
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NotificationTypeNewMessage {
    /// The message
    pub message: crate::types::Message,
    /// True, if message content must be displayed in notifications
    pub show_preview: bool,
}

/// New secret chat was created
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NotificationTypeNewSecretChat {
}

/// New message was received through a push notification
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NotificationTypeNewPushMessage {
    /// The message identifier. The message will not be available in the chat history, but the identifier can be used in viewMessages, or as a message to be replied in the same chat
    pub message_id: i64,
    /// Identifier of the sender of the message. Corresponding user or chat may be inaccessible
    pub sender_id: crate::enums::MessageSender,
    /// Name of the sender
    pub sender_name: String,
    /// True, if the message is outgoing
    pub is_outgoing: bool,
    /// Push message content
    pub content: crate::enums::PushMessageContent,
}

/// A group containing notifications of type notificationTypeNewMessage and notificationTypeNewPushMessage with ordinary unread messages
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NotificationGroupTypeMessages {
}

/// A group containing notifications of type notificationTypeNewMessage and notificationTypeNewPushMessage with unread mentions of the current user, replies to their messages, or a pinned message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NotificationGroupTypeMentions {
}

/// A group containing a notification of type notificationTypeNewSecretChat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NotificationGroupTypeSecretChat {
}

/// Describes a notification sound in MP3 format
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NotificationSound {
    /// Unique identifier of the notification sound
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Duration of the sound, in seconds
    pub duration: i32,
    /// Point in time (Unix timestamp) when the sound was created
    pub date: i32,
    /// Title of the notification sound
    pub title: String,
    /// Arbitrary data, defined while the sound was uploaded
    pub data: String,
    /// File containing the sound
    pub sound: crate::types::File,
}

/// Contains a list of notification sounds
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NotificationSounds {
    /// A list of notification sounds
    pub notification_sounds: Vec<crate::types::NotificationSound>,
}

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
    #[serde(rename = "type")]
    pub r#type: crate::enums::NotificationType,
}

/// Describes a group of notifications
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NotificationGroup {
    /// Unique persistent auto-incremented from 1 identifier of the notification group
    pub id: i32,
    /// Type of the group
    #[serde(rename = "type")]
    pub r#type: crate::enums::NotificationGroupType,
    /// Identifier of a chat to which all notifications in the group belong
    pub chat_id: i64,
    /// Total number of active notifications in the group
    pub total_count: i32,
    /// The list of active notifications
    pub notifications: Vec<crate::types::Notification>,
}

/// The notification settings section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionNotifications {
    /// Subsection of the section; may be one of
    /// "", "accounts", "private-chats", "private-chats/edit", "private-chats/show", "private-chats/preview",
    /// "private-chats/sound", "private-chats/add-exception", "private-chats/delete-exceptions",
    /// "private-chats/light-color", "private-chats/vibrate", "private-chats/priority", "groups", "groups/edit",
    /// "groups/show", "groups/preview", "groups/sound", "groups/add-exception", "groups/delete-exceptions",
    /// "groups/light-color", "groups/vibrate", "groups/priority", "channels", "channels/edit", "channels/show",
    /// "channels/preview", "channels/sound", "channels/add-exception", "channels/delete-exceptions",
    /// "channels/light-color", "channels/vibrate", "channels/priority", "stories", "stories/new", "stories/important",
    /// "stories/show-sender", "stories/sound", "stories/add-exception", "stories/delete-exceptions",
    /// "stories/light-color", "stories/vibrate", "stories/priority", "reactions", "reactions/messages",
    /// "reactions/stories", "reactions/show-sender", "reactions/sound", "reactions/light-color", "reactions/vibrate",
    /// "reactions/priority", "in-app-sounds", "in-app-vibrate", "in-app-preview", "in-chat-sounds", "in-app-popup",
    /// "lock-screen-names", "include-channels", "include-muted-chats", "count-unread-messages", "new-contacts",
    /// "pinned-messages", "reset", "web"
    pub subsection: String,
}

/// The file is a notification sound
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FileTypeNotificationSound {
}

/// Notification settings for a chat were changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatNotificationSettings {
    /// Chat identifier
    pub chat_id: i64,
    /// The new notification settings
    pub notification_settings: crate::types::ChatNotificationSettings,
}

/// The value of the default disable_notification parameter, used when a message is sent to the chat, was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatDefaultDisableNotification {
    /// Chat identifier
    pub chat_id: i64,
    /// The new default_disable_notification value
    pub default_disable_notification: bool,
}

/// Notification settings for some type of chats were updated
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateScopeNotificationSettings {
    /// Types of chats for which notification settings were updated
    pub scope: crate::enums::NotificationSettingsScope,
    /// The new notification settings
    pub notification_settings: crate::types::ScopeNotificationSettings,
}

/// Notification settings for reactions were updated
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateReactionNotificationSettings {
    /// The new notification settings
    pub notification_settings: crate::types::ReactionNotificationSettings,
}

/// A notification was changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateNotification {
    /// Unique notification group identifier
    pub notification_group_id: i32,
    /// Changed notification
    pub notification: crate::types::Notification,
}

/// A list of active notifications in a notification group has changed
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateNotificationGroup {
    /// Unique notification group identifier
    pub notification_group_id: i32,
    /// New type of the notification group
    #[serde(rename = "type")]
    pub r#type: crate::enums::NotificationGroupType,
    /// Identifier of a chat to which all notifications in the group belong
    pub chat_id: i64,
    /// Chat identifier, which notification settings must be applied to the added notifications
    pub notification_settings_chat_id: i64,
    /// Identifier of the notification sound to be played; 0 if sound is disabled
    #[serde_as(as = "DisplayFromStr")]
    pub notification_sound_id: i64,
    /// Total number of unread notifications in the group, can be bigger than number of active notifications
    pub total_count: i32,
    /// List of added group notifications, sorted by notification identifier
    pub added_notifications: Vec<crate::types::Notification>,
    /// Identifiers of removed group notifications, sorted by notification identifier
    pub removed_notification_ids: Vec<i32>,
}

/// Contains active notifications that were shown on previous application launches. This update is sent only if the message database is used. In that case it comes once before any updateNotification and updateNotificationGroup update
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateActiveNotifications {
    /// Lists of active notification groups
    pub groups: Vec<crate::types::NotificationGroup>,
}

/// Describes whether there are some pending notification updates. Can be used to prevent application from killing, while there are some pending notifications
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateHavePendingNotifications {
    /// True, if there are some delayed notification updates, which will be sent soon
    pub have_delayed_notifications: bool,
    /// True, if there can be some yet unreceived notifications, which are being fetched from the server
    pub have_unreceived_notifications: bool,
}

/// A service notification from the server was received. Upon receiving this the application must show a popup with the content of the notification
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateServiceNotification {
    /// Notification type. If type begins with "AUTH_KEY_DROP_", then two buttons "Cancel" and "Log out" must be shown under notification; if user presses the second, all local data must be destroyed using Destroy method
    #[serde(rename = "type")]
    pub r#type: String,
    /// Notification content
    pub content: crate::enums::MessageContent,
}

/// The list of saved notification sounds was updated. This update may not be sent until information about a notification sound was requested for the first time
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateSavedNotificationSounds {
    /// The new list of identifiers of saved notification sounds
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub notification_sound_ids: Vec<i64>,
}

/// Download or upload file speed for the user was limited, but it can be restored by subscription to Telegram Premium. The notification can be postponed until a file being downloaded or uploaded is visible to the user.
/// Use getOption("premium_download_speedup") or getOption("premium_upload_speedup") to get expected speedup after subscription to Telegram Premium
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateSpeedLimitNotification {
    /// True, if upload speed was limited; false, if download speed was limited
    pub is_upload: bool,
}

