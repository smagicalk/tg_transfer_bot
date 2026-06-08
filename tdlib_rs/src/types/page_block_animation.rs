#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// An animation
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockAnimation {
    /// Animation file; may be null
    pub animation: Option<crate::types::Animation>,
    /// Animation caption
    pub caption: crate::types::PageBlockCaption,
    /// True, if the animation must be played automatically
    pub need_autoplay: bool,
}
