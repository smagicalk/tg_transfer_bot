#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a link to a sticker
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeSticker {
    /// The sticker. It can be an arbitrary WEBP image and can have dimensions bigger than 512
    pub sticker: crate::types::Sticker,
}
