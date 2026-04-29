#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a link to a sticker set. Call searchStickerSet with the given sticker set name to process the link and show the sticker set.
/// If the sticker set is found and the user wants to add it, then call changeStickerSet
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeStickerSet {
    /// Name of the sticker set
    pub sticker_set_name: String,
    /// True, if the sticker set is expected to contain custom emoji
    pub expect_custom_emoji: bool,
}
