#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A URL linking to a sticker set
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TmeUrlTypeStickerSet {
    /// Identifier of the sticker set
    #[serde_as(as = "DisplayFromStr")]
    pub sticker_set_id: i64,
}
