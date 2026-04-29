#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmojiCategory {
    /// Describes an emoji category
    #[serde(rename(serialize = "emojiCategory", deserialize = "emojiCategory"))]
    EmojiCategory(crate::types::EmojiCategory),
}
