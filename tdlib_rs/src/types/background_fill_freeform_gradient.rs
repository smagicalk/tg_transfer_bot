#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a freeform gradient fill of a background
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BackgroundFillFreeformGradient {
    /// A list of 3 or 4 colors of the freeform gradient in the RGB format
    pub colors: Vec<i32>,
}
