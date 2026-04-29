#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GameHighScore {
    /// Contains one row of the game high score table
    #[serde(rename(serialize = "gameHighScore", deserialize = "gameHighScore"))]
    GameHighScore(crate::types::GameHighScore),
}
