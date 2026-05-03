#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputChatPhoto {
    /// A previously used profile photo of the current user
    #[serde(rename(
        serialize = "inputChatPhotoPrevious",
        deserialize = "inputChatPhotoPrevious"
    ))]
    Previous(crate::types::InputChatPhotoPrevious),
    /// A static photo in JPEG format
    #[serde(rename(
        serialize = "inputChatPhotoStatic",
        deserialize = "inputChatPhotoStatic"
    ))]
    Static(crate::types::InputChatPhotoStatic),
    /// An animation in MPEG4 format; must be square, at most 10 seconds long, have width between 160 and 1280 and be at most 2MB in size
    #[serde(rename(
        serialize = "inputChatPhotoAnimation",
        deserialize = "inputChatPhotoAnimation"
    ))]
    Animation(crate::types::InputChatPhotoAnimation),
    /// A sticker on a custom background
    #[serde(rename(
        serialize = "inputChatPhotoSticker",
        deserialize = "inputChatPhotoSticker"
    ))]
    Sticker(crate::types::InputChatPhotoSticker),
}
