#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A message with a game
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageGame {
    /// The game description
    pub game: crate::types::Game,
}
