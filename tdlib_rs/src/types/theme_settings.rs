#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes theme settings
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ThemeSettings {
    /// Base theme for this theme
    pub base_theme: crate::enums::BuiltInTheme,
    /// Theme accent color in ARGB format
    pub accent_color: i32,
    /// The background to be used in chats; may be null
    pub background: Option<crate::types::Background>,
    /// The fill to be used as a background for outgoing messages; may be null if the fill from the base theme must be used instead
    pub outgoing_message_fill: Option<crate::enums::BackgroundFill>,
    /// If true, the freeform gradient fill needs to be animated on every sent message
    pub animate_outgoing_message_fill: bool,
    /// Accent color of outgoing messages in ARGB format
    pub outgoing_message_accent_color: i32,
}
