#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A custom emoji set as emoji status
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmojiStatusTypeCustomEmoji {
    /// Identifier of the custom emoji in stickerFormatTgs format
    #[serde_as(as = "DisplayFromStr")]
    pub custom_emoji_id: i64,
}
