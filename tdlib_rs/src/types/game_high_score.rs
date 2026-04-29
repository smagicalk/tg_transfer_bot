#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains one row of the game high score table
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GameHighScore {
    /// Position in the high score table
    pub position: i32,
    /// User identifier
    pub user_id: i64,
    /// User score
    pub score: i32,
}
