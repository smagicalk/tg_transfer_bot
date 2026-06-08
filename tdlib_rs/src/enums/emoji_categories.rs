#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmojiCategories {
    /// Represents a list of emoji categories
    #[serde(rename(serialize = "emojiCategories", deserialize = "emojiCategories"))]
    EmojiCategories(crate::types::EmojiCategories),
}
