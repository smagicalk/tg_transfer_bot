#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A message with a game
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentGame {
    /// Game title, empty for pinned game message
    pub title: String,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}
