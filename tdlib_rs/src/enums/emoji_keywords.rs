#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmojiKeywords {
    /// Represents a list of emojis with their keywords
    #[serde(rename(serialize = "emojiKeywords", deserialize = "emojiKeywords"))]
    EmojiKeywords(crate::types::EmojiKeywords),
}
