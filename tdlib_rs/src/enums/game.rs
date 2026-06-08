#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Game {
    /// Describes a game. Use getInternalLink with internalLinkTypeGame to share the game
    #[serde(rename(serialize = "game", deserialize = "game"))]
    Game(crate::types::Game),
}
