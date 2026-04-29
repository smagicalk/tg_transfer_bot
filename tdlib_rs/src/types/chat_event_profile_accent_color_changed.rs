#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The chat's profile accent color or profile background custom emoji were changed
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventProfileAccentColorChanged {
    /// Previous identifier of chat's profile accent color; -1 if none
    pub old_profile_accent_color_id: i32,
    /// Previous identifier of the custom emoji; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub old_profile_background_custom_emoji_id: i64,
    /// New identifier of chat's profile accent color; -1 if none
    pub new_profile_accent_color_id: i32,
    /// New identifier of the custom emoji; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub new_profile_background_custom_emoji_id: i64,
}
