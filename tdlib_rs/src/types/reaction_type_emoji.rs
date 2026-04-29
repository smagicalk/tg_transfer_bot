#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A reaction with an emoji
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReactionTypeEmoji {
    /// Text representation of the reaction
    pub emoji: String,
}
