#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a chat theme based on an upgraded gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GiftChatTheme {
    /// The gift
    pub gift: crate::types::UpgradedGift,
    /// Theme settings for a light chat theme
    pub light_settings: crate::types::ThemeSettings,
    /// Theme settings for a dark chat theme
    pub dark_settings: crate::types::ThemeSettings,
}
