#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A map
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockMap {
    /// Location of the map center
    pub location: crate::types::Location,
    /// Map zoom level
    pub zoom: i32,
    /// Map width
    pub width: i32,
    /// Map height
    pub height: i32,
    /// Block caption
    pub caption: crate::types::PageBlockCaption,
}
