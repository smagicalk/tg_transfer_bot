#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A table
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockTable {
    /// Table caption
    pub caption: crate::enums::RichText,
    /// Table cells
    pub cells: Vec<Vec<crate::types::PageBlockTableCell>>,
    /// True, if the table is bordered
    pub is_bordered: bool,
    /// True, if the table is striped
    pub is_striped: bool,
}
