#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes the button that opens a private chat with the bot and sends a start message to the bot with the given parameter
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultsButtonTypeStartBot {
    /// The parameter for the bot start message
    pub parameter: String,
}
