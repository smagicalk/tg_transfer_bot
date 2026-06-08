#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about supported accent color for user/chat name, background of empty chat photo, replies to messages and link previews
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AccentColor {
    /// Accent color identifier
    pub id: i32,
    /// Identifier of a built-in color to use in places, where only one color is needed; 0-6
    pub built_in_accent_color_id: i32,
    /// The list of 1-3 colors in RGB format, describing the accent color, as expected to be shown in light themes
    pub light_theme_colors: Vec<i32>,
    /// The list of 1-3 colors in RGB format, describing the accent color, as expected to be shown in dark themes
    pub dark_theme_colors: Vec<i32>,
    /// The minimum chat boost level required to use the color in a channel chat
    pub min_channel_chat_boost_level: i32,
}
