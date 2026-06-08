#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A video
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockVideo {
    /// Video file; may be null
    pub video: Option<crate::types::Video>,
    /// Video caption
    pub caption: crate::types::PageBlockCaption,
    /// True, if the video must be played automatically
    pub need_autoplay: bool,
    /// True, if the video must be looped
    pub is_looped: bool,
}
