#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a custom keyboard layout to quickly reply to bots
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReplyMarkupShowKeyboard {
    /// A list of rows of bot keyboard buttons
    pub rows: Vec<Vec<crate::types::KeyboardButton>>,
    /// True, if the keyboard is expected to always be shown when the ordinary keyboard is hidden
    pub is_persistent: bool,
    /// True, if the application needs to resize the keyboard vertically
    pub resize_keyboard: bool,
    /// True, if the application needs to hide the keyboard after use
    pub one_time: bool,
    /// True, if the keyboard must automatically be shown to the current user. For outgoing messages, specify true to show the keyboard only for the mentioned users and for the target user of a reply
    pub is_personal: bool,
    /// If non-empty, the placeholder to be shown in the input field when the keyboard is active; 0-64 characters
    pub input_field_placeholder: String,
}
