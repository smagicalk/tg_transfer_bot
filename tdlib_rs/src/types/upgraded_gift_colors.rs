#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about color scheme for user's name, background of empty chat photo, replies to messages and link previews
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftColors {
    /// Unique identifier of the upgraded gift colors
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Custom emoji identifier of the model of the upgraded gift
    #[serde_as(as = "DisplayFromStr")]
    pub model_custom_emoji_id: i64,
    /// Custom emoji identifier of the symbol of the upgraded gift
    #[serde_as(as = "DisplayFromStr")]
    pub symbol_custom_emoji_id: i64,
    /// Accent color to use in light themes in RGB format
    pub light_theme_accent_color: i32,
    /// The list of 1-3 colors in RGB format, describing the accent color, as expected to be shown in light themes
    pub light_theme_colors: Vec<i32>,
    /// Accent color to use in dark themes in RGB format
    pub dark_theme_accent_color: i32,
    /// The list of 1-3 colors in RGB format, describing the accent color, as expected to be shown in dark themes
    pub dark_theme_colors: Vec<i32>,
}
