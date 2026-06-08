#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A button that forces an inline query to the bot to be inserted in the input field
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InlineKeyboardButtonTypeSwitchInline {
    /// Inline query to be sent to the bot
    pub query: String,
    /// Target chat from which to send the inline query
    pub target_chat: crate::enums::TargetChat,
}
