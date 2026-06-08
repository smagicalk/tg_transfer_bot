#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about supported accent color for user profile photo background
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ProfileAccentColor {
    /// Profile accent color identifier
    pub id: i32,
    /// Accent colors expected to be used in light themes
    pub light_theme_colors: crate::types::ProfileAccentColors,
    /// Accent colors expected to be used in dark themes
    pub dark_theme_colors: crate::types::ProfileAccentColors,
    /// The minimum chat boost level required to use the color in a supergroup chat
    pub min_supergroup_chat_boost_level: i32,
    /// The minimum chat boost level required to use the color in a channel chat
    pub min_channel_chat_boost_level: i32,
}
