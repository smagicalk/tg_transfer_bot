#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmojiStatus {
    /// Describes an emoji to be shown instead of the Telegram Premium badge
    #[serde(rename(serialize = "emojiStatus", deserialize = "emojiStatus"))]
    EmojiStatus(crate::types::EmojiStatus),
}
