#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is a link to an animation player
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeEmbeddedAnimationPlayer {
    /// URL of the external animation player
    pub url: String,
    /// The cached animation; may be null if unknown
    pub animation: Option<crate::types::Animation>,
    /// Thumbnail of the animation; may be null if unknown
    pub thumbnail: Option<crate::types::Photo>,
    /// Duration of the animation, in seconds
    pub duration: i32,
    /// Expected width of the embedded player
    pub width: i32,
    /// Expected height of the embedded player
    pub height: i32,
}
