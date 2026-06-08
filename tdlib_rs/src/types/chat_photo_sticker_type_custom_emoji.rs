#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Information about the custom emoji, which was used to create the chat photo
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatPhotoStickerTypeCustomEmoji {
    /// Identifier of the custom emoji
    #[serde_as(as = "DisplayFromStr")]
    pub custom_emoji_id: i64,
}
