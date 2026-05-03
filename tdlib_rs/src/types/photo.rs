#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a photo
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Photo {
    /// True, if stickers were added to the photo. The list of corresponding sticker sets can be received using getAttachedStickerSets
    pub has_stickers: bool,
    /// Photo minithumbnail; may be null
    pub minithumbnail: Option<crate::types::Minithumbnail>,
    /// Available variants of the photo, in different sizes
    pub sizes: Vec<crate::types::PhotoSize>,
}
