#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a chat or user profile photo
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatPhoto {
    /// Unique photo identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Point in time (Unix timestamp) when the photo has been added
    pub added_date: i32,
    /// Photo minithumbnail; may be null
    pub minithumbnail: Option<crate::types::Minithumbnail>,
    /// Available variants of the photo in JPEG format, in different size
    pub sizes: Vec<crate::types::PhotoSize>,
    /// A big (up to 1280x1280) animated variant of the photo in MPEG4 format; may be null
    pub animation: Option<crate::types::AnimatedChatPhoto>,
    /// A small (160x160) animated variant of the photo in MPEG4 format; may be null even if the big animation is available
    pub small_animation: Option<crate::types::AnimatedChatPhoto>,
    /// Sticker-based version of the chat photo; may be null
    pub sticker: Option<crate::types::ChatPhotoSticker>,
}
