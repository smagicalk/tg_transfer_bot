#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The category contains a list of similar emoji to search for in getStickers and searchStickers for stickers,
/// or getInlineQueryResults with the bot getOption("animation_search_bot_username") for animations
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmojiCategorySourceSearch {
    /// List of emojis to search for
    pub emojis: Vec<String>,
}
