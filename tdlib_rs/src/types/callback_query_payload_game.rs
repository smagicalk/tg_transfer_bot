#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The payload for a game callback button
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallbackQueryPayloadGame {
    /// A short name of the game that was attached to the callback button
    pub game_short_name: String,
}
