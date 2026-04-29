#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A message with a game; not supported for channels or secret chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessageGame {
    /// User identifier of the bot that owns the game
    pub bot_user_id: i64,
    /// Short name of the game
    pub game_short_name: String,
}
