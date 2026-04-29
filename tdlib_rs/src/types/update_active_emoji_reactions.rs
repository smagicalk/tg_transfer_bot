#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The list of active emoji reactions has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateActiveEmojiReactions {
    /// The new list of active emoji reactions
    pub emojis: Vec<String>,
}
