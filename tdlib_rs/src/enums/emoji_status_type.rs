#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmojiStatusType {
    /// A custom emoji set as emoji status
    #[serde(rename(
        serialize = "emojiStatusTypeCustomEmoji",
        deserialize = "emojiStatusTypeCustomEmoji"
    ))]
    CustomEmoji(crate::types::EmojiStatusTypeCustomEmoji),
    /// An upgraded gift set as emoji status
    #[serde(rename(
        serialize = "emojiStatusTypeUpgradedGift",
        deserialize = "emojiStatusTypeUpgradedGift"
    ))]
    UpgradedGift(crate::types::EmojiStatusTypeUpgradedGift),
}
