#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmojiCategorySource {
    /// The category contains a list of similar emoji to search for in getStickers and searchStickers for stickers,
    /// or getInlineQueryResults with the bot getOption("animation_search_bot_username") for animations
    #[serde(rename(
        serialize = "emojiCategorySourceSearch",
        deserialize = "emojiCategorySourceSearch"
    ))]
    Search(crate::types::EmojiCategorySourceSearch),
    /// The category contains premium stickers that must be found by getPremiumStickers
    #[serde(rename(
        serialize = "emojiCategorySourcePremium",
        deserialize = "emojiCategorySourcePremium"
    ))]
    Premium,
}
