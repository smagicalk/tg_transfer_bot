#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A collage
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockCollage {
    /// Collage item contents
    pub page_blocks: Vec<crate::enums::PageBlock>,
    /// Block caption
    pub caption: crate::types::PageBlockCaption,
}
