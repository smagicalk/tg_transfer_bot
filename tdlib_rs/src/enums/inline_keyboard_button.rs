#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InlineKeyboardButton {
    /// Represents a single button in an inline keyboard
    #[serde(rename(
        serialize = "inlineKeyboardButton",
        deserialize = "inlineKeyboardButton"
    ))]
    InlineKeyboardButton(crate::types::InlineKeyboardButton),
}
