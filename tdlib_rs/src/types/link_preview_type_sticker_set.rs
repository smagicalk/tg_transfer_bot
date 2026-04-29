#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a link to a sticker set
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeStickerSet {
    /// Up to 4 stickers from the sticker set
    pub stickers: Vec<crate::types::Sticker>,
}
