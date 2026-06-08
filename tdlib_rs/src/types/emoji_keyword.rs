#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents an emoji with its keyword
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmojiKeyword {
    /// The emoji
    pub emoji: String,
    /// The keyword
    pub keyword: String,
}
