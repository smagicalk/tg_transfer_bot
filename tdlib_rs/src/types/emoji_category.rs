#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes an emoji category
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EmojiCategory {
    /// Name of the category
    pub name: String,
    /// Custom emoji sticker, which represents icon of the category
    pub icon: crate::types::Sticker,
    /// Source of stickers for the emoji category
    pub source: crate::enums::EmojiCategorySource,
    /// True, if the category must be shown first when choosing a sticker for the start page
    pub is_greeting: bool,
}
