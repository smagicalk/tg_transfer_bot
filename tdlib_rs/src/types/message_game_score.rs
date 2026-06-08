#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A new high score was achieved in a game
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageGameScore {
    /// Identifier of the message with the game, can be an identifier of a deleted message
    pub game_message_id: i64,
    /// Identifier of the game; may be different from the games presented in the message with the game
    #[serde_as(as = "DisplayFromStr")]
    pub game_id: i64,
    /// New score
    pub score: i32,
}
