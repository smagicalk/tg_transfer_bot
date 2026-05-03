#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a list of emojis with their keywords
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmojiKeywords {
    /// List of emojis with their keywords
    pub emoji_keywords: Vec<crate::types::EmojiKeyword>,
}
