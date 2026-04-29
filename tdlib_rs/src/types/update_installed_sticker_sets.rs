#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The list of installed sticker sets was updated
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateInstalledStickerSets {
    /// Type of the affected stickers
    pub sticker_type: crate::enums::StickerType,
    /// The new list of installed ordinary sticker sets
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub sticker_set_ids: Vec<i64>,
}
