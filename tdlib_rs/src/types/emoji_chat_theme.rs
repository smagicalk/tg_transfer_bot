#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a chat theme based on an emoji
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EmojiChatTheme {
    /// Theme name
    pub name: String,
    /// Theme settings for a light chat theme
    pub light_settings: crate::types::ThemeSettings,
    /// Theme settings for a dark chat theme
    pub dark_settings: crate::types::ThemeSettings,
}
