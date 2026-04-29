#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GameHighScores {
    /// Contains a list of game high scores
    #[serde(rename(serialize = "gameHighScores", deserialize = "gameHighScores"))]
    GameHighScores(crate::types::GameHighScores),
}
