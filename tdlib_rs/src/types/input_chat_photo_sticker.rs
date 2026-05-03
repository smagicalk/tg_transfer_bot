#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A sticker on a custom background
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputChatPhotoSticker {
    /// Information about the sticker
    pub sticker: crate::types::ChatPhotoSticker,
}
