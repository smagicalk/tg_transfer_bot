#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A photo message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessagePhoto {
    /// The photo
    pub photo: crate::types::Photo,
    /// Photo caption
    pub caption: crate::types::FormattedText,
    /// True, if the caption must be shown above the photo; otherwise, the caption must be shown below the photo
    pub show_caption_above_media: bool,
    /// True, if the photo preview must be covered by a spoiler animation
    pub has_spoiler: bool,
    /// True, if the photo must be blurred and must be shown only while tapped
    pub is_secret: bool,
}
