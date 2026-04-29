#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A list of data blocks
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PageBlockList {
    /// The items of the list
    pub items: Vec<crate::types::PageBlockListItem>,
}
