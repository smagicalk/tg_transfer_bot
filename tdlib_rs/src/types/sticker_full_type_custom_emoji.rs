#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The sticker is a custom emoji to be used inside message text and caption. Currently, only Telegram Premium users can use custom emoji
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StickerFullTypeCustomEmoji {
    /// Identifier of the custom emoji
    #[serde_as(as = "DisplayFromStr")]
    pub custom_emoji_id: i64,
    /// True, if the sticker must be repainted to a text color in messages, the color of the Telegram Premium badge in emoji status, white color on chat photos, or another appropriate color in other places
    pub needs_repainting: bool,
}
