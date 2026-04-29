#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The chat accent color or background custom emoji were changed
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventAccentColorChanged {
    /// Previous identifier of chat accent color
    pub old_accent_color_id: i32,
    /// Previous identifier of the custom emoji; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub old_background_custom_emoji_id: i64,
    /// New identifier of chat accent color
    pub new_accent_color_id: i32,
    /// New identifier of the custom emoji; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub new_background_custom_emoji_id: i64,
}
