#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmojiStatuses {
    /// Contains a list of emoji statuses
    #[serde(rename(serialize = "emojiStatuses", deserialize = "emojiStatuses"))]
    EmojiStatuses(crate::types::EmojiStatuses),
}
