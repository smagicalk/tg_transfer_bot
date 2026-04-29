#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a solid fill of a background
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BackgroundFillSolid {
    /// A color of the background in the RGB format
    pub color: i32,
}
