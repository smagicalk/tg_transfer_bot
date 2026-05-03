#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A dice message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessageDice {
    /// Emoji on which the dice throw animation is based
    pub emoji: String,
    /// True, if the chat message draft must be deleted
    pub clear_draft: bool,
}
