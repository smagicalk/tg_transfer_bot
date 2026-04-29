#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A custom emoji. The text behind a custom emoji must be an emoji. Only premium users can use premium custom emoji
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypeCustomEmoji {
    /// Unique identifier of the custom emoji
    #[serde_as(as = "DisplayFromStr")]
    pub custom_emoji_id: i64,
}
