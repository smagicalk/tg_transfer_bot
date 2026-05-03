#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum KeyboardButton {
    /// Represents a single button in a bot keyboard
    #[serde(rename(serialize = "keyboardButton", deserialize = "keyboardButton"))]
    KeyboardButton(crate::types::KeyboardButton),
}
