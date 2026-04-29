#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Information about the sticker, which was used to create the chat photo
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatPhotoStickerTypeRegularOrMask {
    /// Sticker set identifier
    #[serde_as(as = "DisplayFromStr")]
    pub sticker_set_id: i64,
    /// Identifier of the sticker in the set
    #[serde_as(as = "DisplayFromStr")]
    pub sticker_id: i64,
}
