#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


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
