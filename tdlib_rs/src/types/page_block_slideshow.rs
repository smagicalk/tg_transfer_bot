#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A slideshow
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockSlideshow {
    /// Slideshow item contents
    pub page_blocks: Vec<crate::enums::PageBlock>,
    /// Block caption
    pub caption: crate::types::PageBlockCaption,
}
