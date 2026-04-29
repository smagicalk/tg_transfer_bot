#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of emoji statuses
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmojiStatuses {
    /// The list of emoji statuses identifiers
    pub emoji_statuses: Vec<crate::types::EmojiStatus>,
}
