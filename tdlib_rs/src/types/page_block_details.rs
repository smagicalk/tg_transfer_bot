#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A collapsible block
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockDetails {
    /// Always visible heading for the block
    pub header: crate::enums::RichText,
    /// Block contents
    pub page_blocks: Vec<crate::enums::PageBlock>,
    /// True, if the block is open by default
    pub is_open: bool,
}
