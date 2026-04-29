#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of features available on the first chat boost levels
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatBoostFeatures {
    /// The list of features
    pub features: Vec<crate::types::ChatBoostLevelFeatures>,
    /// The minimum boost level required to set custom emoji for profile background
    pub min_profile_background_custom_emoji_boost_level: i32,
    /// The minimum boost level required to set custom emoji for reply header and link preview background; for channel chats only
    pub min_background_custom_emoji_boost_level: i32,
    /// The minimum boost level required to set emoji status
    pub min_emoji_status_boost_level: i32,
    /// The minimum boost level required to set a chat theme background as chat background
    pub min_chat_theme_background_boost_level: i32,
    /// The minimum boost level required to set custom chat background
    pub min_custom_background_boost_level: i32,
    /// The minimum boost level required to set custom emoji sticker set for the chat; for supergroup chats only
    pub min_custom_emoji_sticker_set_boost_level: i32,
    /// The minimum boost level allowing to enable automatic translation of messages for non-Premium users; for channel chats only
    pub min_automatic_translation_boost_level: i32,
    /// The minimum boost level allowing to recognize speech in video note and voice note messages for non-Premium users; for supergroup chats only
    pub min_speech_recognition_boost_level: i32,
    /// The minimum boost level allowing to disable sponsored messages in the chat; for channel chats only
    pub min_sponsored_message_disable_boost_level: i32,
}
