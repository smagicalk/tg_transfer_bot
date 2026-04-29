#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a link to a game. Call searchPublicChat with the given bot username, check that the user is a bot,
/// ask the current user to select a chat to send the game, and then call sendMessage with inputMessageGame
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeGame {
    /// Username of the bot that owns the game
    pub bot_username: String,
    /// Short name of the game
    pub game_short_name: String,
}
