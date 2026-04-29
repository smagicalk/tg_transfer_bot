#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a link to a chat with a Telegram bot. Call searchPublicChat with the given bot username, check that the user is a bot, show START button in the chat with the bot,
/// and then call sendBotStartMessage with the given start parameter after the button is pressed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeBotStart {
    /// Username of the bot
    pub bot_username: String,
    /// The parameter to be passed to sendBotStartMessage
    pub start_parameter: String,
    /// True, if sendBotStartMessage must be called automatically without showing the START button
    pub autostart: bool,
}
