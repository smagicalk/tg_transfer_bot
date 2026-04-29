#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a cell of a table
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockTableCell {
    /// Cell text; may be null. If the text is null, then the cell must be invisible
    pub text: Option<crate::enums::RichText>,
    /// True, if it is a header cell
    pub is_header: bool,
    /// The number of columns the cell spans
    pub colspan: i32,
    /// The number of rows the cell spans
    pub rowspan: i32,
    /// Horizontal cell content alignment
    pub align: crate::enums::PageBlockHorizontalAlignment,
    /// Vertical cell content alignment
    pub valign: crate::enums::PageBlockVerticalAlignment,
}
