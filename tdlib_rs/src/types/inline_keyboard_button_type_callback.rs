#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A button that sends a callback query to a bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InlineKeyboardButtonTypeCallback {
    /// Data to be sent to the bot via a callback query
    pub data: String,
}
