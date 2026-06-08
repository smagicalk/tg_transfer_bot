#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PageBlockTableCell {
    /// Represents a cell of a table
    #[serde(rename(serialize = "pageBlockTableCell", deserialize = "pageBlockTableCell"))]
    PageBlockTableCell(crate::types::PageBlockTableCell),
}
