#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmojiStatusCustomEmojis {
    /// Contains a list of custom emoji identifiers for emoji statuses
    #[serde(rename(serialize = "emojiStatusCustomEmojis", deserialize = "emojiStatusCustomEmojis"))]
    EmojiStatusCustomEmojis(crate::types::EmojiStatusCustomEmojis),
}
