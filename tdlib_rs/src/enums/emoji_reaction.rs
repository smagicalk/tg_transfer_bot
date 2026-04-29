#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmojiReaction {
    /// Contains information about an emoji reaction
    #[serde(rename(serialize = "emojiReaction", deserialize = "emojiReaction"))]
    EmojiReaction(crate::types::EmojiReaction),
}
