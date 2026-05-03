#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains an inline keyboard layout
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReplyMarkupInlineKeyboard {
    /// A list of rows of inline keyboard buttons
    pub rows: Vec<Vec<crate::types::InlineKeyboardButton>>,
}
