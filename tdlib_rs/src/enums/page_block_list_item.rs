#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PageBlockListItem {
    /// Describes an item of a list page block
    #[serde(rename(serialize = "pageBlockListItem", deserialize = "pageBlockListItem"))]
    PageBlockListItem(crate::types::PageBlockListItem),
}
