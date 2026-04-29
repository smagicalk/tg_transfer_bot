#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatPhotoStickerType {
    /// Information about the sticker, which was used to create the chat photo
    #[serde(rename(serialize = "chatPhotoStickerTypeRegularOrMask", deserialize = "chatPhotoStickerTypeRegularOrMask"))]
    RegularOrMask(crate::types::ChatPhotoStickerTypeRegularOrMask),
    /// Information about the custom emoji, which was used to create the chat photo
    #[serde(rename(serialize = "chatPhotoStickerTypeCustomEmoji", deserialize = "chatPhotoStickerTypeCustomEmoji"))]
    CustomEmoji(crate::types::ChatPhotoStickerTypeCustomEmoji),
}
