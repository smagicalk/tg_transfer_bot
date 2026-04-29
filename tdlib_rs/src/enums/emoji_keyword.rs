#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmojiKeyword {
    /// Represents an emoji with its keyword
    #[serde(rename(serialize = "emojiKeyword", deserialize = "emojiKeyword"))]
    EmojiKeyword(crate::types::EmojiKeyword),
}
