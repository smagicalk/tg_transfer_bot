#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about notification settings for several chats
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ScopeNotificationSettings {
    /// Time left before notifications will be unmuted, in seconds
    pub mute_for: i32,
    /// Identifier of the notification sound to be played; 0 if sound is disabled
    #[serde_as(as = "DisplayFromStr")]
    pub sound_id: i64,
    /// True, if message content must be displayed in notifications
    pub show_preview: bool,
    /// If true, story notifications are received only for the first 5 chats from topChatCategoryUsers regardless of the value of mute_stories
    pub use_default_mute_stories: bool,
    /// True, if story notifications are disabled
    pub mute_stories: bool,
    /// Identifier of the notification sound to be played for stories; 0 if sound is disabled
    #[serde_as(as = "DisplayFromStr")]
    pub story_sound_id: i64,
    /// True, if the chat that posted a story must be displayed in notifications
    pub show_story_poster: bool,
    /// True, if notifications for incoming pinned messages will be created as for an ordinary unread message
    pub disable_pinned_message_notifications: bool,
    /// True, if notifications for messages with mentions will be created as for an ordinary unread message
    pub disable_mention_notifications: bool,
}
