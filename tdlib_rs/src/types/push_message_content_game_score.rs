#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A new high score was achieved in a game
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentGameScore {
    /// Game title, empty for pinned message
    pub title: String,
    /// New score, 0 for pinned message
    pub score: i32,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}
