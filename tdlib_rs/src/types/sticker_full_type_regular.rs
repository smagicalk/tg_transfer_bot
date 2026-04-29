#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The sticker is a regular sticker
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StickerFullTypeRegular {
    /// Premium animation of the sticker; may be null. If present, only Telegram Premium users can use the sticker
    pub premium_animation: Option<crate::types::File>,
}
