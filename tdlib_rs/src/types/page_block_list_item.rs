#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes an item of a list page block
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PageBlockListItem {
    /// Item label
    pub label: String,
    /// Item blocks
    pub page_blocks: Vec<crate::enums::PageBlock>,
}
