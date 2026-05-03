#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A button that asks for the 2-step verification password of the current user and then sends a callback query to a bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InlineKeyboardButtonTypeCallbackWithPassword {
    /// Data to be sent to the bot via a callback query
    pub data: String,
}
