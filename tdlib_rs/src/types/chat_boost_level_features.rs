#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of features available on a specific chat boost level
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatBoostLevelFeatures {
    /// Target chat boost level
    pub level: i32,
    /// Number of stories that the chat can publish daily
    pub story_per_day_count: i32,
    /// Number of custom emoji reactions that can be added to the list of available reactions
    pub custom_emoji_reaction_count: i32,
    /// Number of custom colors for chat title
    pub title_color_count: i32,
    /// Number of custom colors for profile photo background
    pub profile_accent_color_count: i32,
    /// True, if custom emoji for profile background can be set
    pub can_set_profile_background_custom_emoji: bool,
    /// Number of custom colors for background of empty chat photo, replies to messages and link previews
    pub accent_color_count: i32,
    /// True, if custom emoji for reply header and link preview background can be set
    pub can_set_background_custom_emoji: bool,
    /// True, if emoji status can be set
    pub can_set_emoji_status: bool,
    /// Number of chat theme backgrounds that can be set as chat background
    pub chat_theme_background_count: i32,
    /// True, if custom background can be set in the chat for all users
    pub can_set_custom_background: bool,
    /// True, if custom emoji sticker set can be set for the chat
    pub can_set_custom_emoji_sticker_set: bool,
    /// True, if automatic translation of messages can be enabled in the chat
    pub can_enable_automatic_translation: bool,
    /// True, if speech recognition can be used for video note and voice note messages by all users
    pub can_recognize_speech: bool,
    /// True, if sponsored messages can be disabled in the chat
    pub can_disable_sponsored_messages: bool,
}
