#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a single button in an inline keyboard
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InlineKeyboardButton {
    /// Text of the button
    pub text: String,
    /// Identifier of the custom emoji that must be shown on the button; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub icon_custom_emoji_id: i64,
    /// Style of the button
    pub style: crate::enums::ButtonStyle,
    /// Type of the button
    pub r#type: crate::enums::InlineKeyboardButtonType,
}
