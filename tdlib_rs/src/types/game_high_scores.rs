#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of game high scores
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GameHighScores {
    /// A list of game high scores
    pub scores: Vec<crate::types::GameHighScore>,
}
