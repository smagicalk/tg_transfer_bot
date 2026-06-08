#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmojiCategoryType {
    /// The category must be used by default (e.g., for custom emoji or animation search)
    #[serde(rename(
        serialize = "emojiCategoryTypeDefault",
        deserialize = "emojiCategoryTypeDefault"
    ))]
    Default,
    /// The category must be used by default for regular sticker selection. It may contain greeting emoji category and premium stickers
    #[serde(rename(
        serialize = "emojiCategoryTypeRegularStickers",
        deserialize = "emojiCategoryTypeRegularStickers"
    ))]
    RegularStickers,
    /// The category must be used for emoji status selection
    #[serde(rename(
        serialize = "emojiCategoryTypeEmojiStatus",
        deserialize = "emojiCategoryTypeEmojiStatus"
    ))]
    EmojiStatus,
    /// The category must be used for chat photo emoji selection
    #[serde(rename(
        serialize = "emojiCategoryTypeChatPhoto",
        deserialize = "emojiCategoryTypeChatPhoto"
    ))]
    ChatPhoto,
}
