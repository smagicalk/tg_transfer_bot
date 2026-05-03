#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Information about the sticker, which was used to create the chat photo. The sticker is shown at the center of the photo and occupies at most 67% of it
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatPhotoSticker {
    /// Type of the sticker
    pub r#type: crate::enums::ChatPhotoStickerType,
    /// The fill to be used as background for the sticker; rotation angle in backgroundFillGradient isn't supported
    pub background_fill: crate::enums::BackgroundFill,
}
