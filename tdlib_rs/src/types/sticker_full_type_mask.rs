#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The sticker is a mask in WEBP format to be placed on photos or videos
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StickerFullTypeMask {
    /// Position where the mask is placed; may be null
    pub mask_position: Option<crate::types::MaskPosition>,
}
