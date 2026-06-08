#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Chat accent colors have changed
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatAccentColors {
    /// Chat identifier
    pub chat_id: i64,
    /// The new chat accent color identifier
    pub accent_color_id: i32,
    /// The new identifier of a custom emoji to be shown on the reply header and link preview background; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub background_custom_emoji_id: i64,
    /// Color scheme based on an upgraded gift to be used for the chat instead of accent_color_id and background_custom_emoji_id; may be null if none
    pub upgraded_gift_colors: Option<crate::types::UpgradedGiftColors>,
    /// The new chat profile accent color identifier; -1 if none
    pub profile_accent_color_id: i32,
    /// The new identifier of a custom emoji to be shown on the profile background; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub profile_background_custom_emoji_id: i64,
}
