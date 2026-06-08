#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// An animation message (GIF-style).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageAnimation {
    /// The animation description
    pub animation: crate::types::Animation,
    /// Animation caption
    pub caption: crate::types::FormattedText,
    /// True, if the caption must be shown above the animation; otherwise, the caption must be shown below the animation
    pub show_caption_above_media: bool,
    /// True, if the animation preview must be covered by a spoiler animation
    pub has_spoiler: bool,
    /// True, if the animation thumbnail must be blurred and the animation must be shown only while tapped
    pub is_secret: bool,
}
