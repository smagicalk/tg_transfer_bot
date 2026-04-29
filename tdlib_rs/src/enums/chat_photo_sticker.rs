#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatPhotoSticker {
    /// Information about the sticker, which was used to create the chat photo. The sticker is shown at the center of the photo and occupies at most 67% of it
    #[serde(rename(serialize = "chatPhotoSticker", deserialize = "chatPhotoSticker"))]
    ChatPhotoSticker(crate::types::ChatPhotoSticker),
}
