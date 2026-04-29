#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents information about a game
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultGame {
    /// Unique identifier of the query result
    pub id: String,
    /// Game result
    pub game: crate::types::Game,
}
