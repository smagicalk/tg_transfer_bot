#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BackgroundFill {
    /// Describes a solid fill of a background
    #[serde(rename(serialize = "backgroundFillSolid", deserialize = "backgroundFillSolid"))]
    Solid(crate::types::BackgroundFillSolid),
    /// Describes a gradient fill of a background
    #[serde(rename(
        serialize = "backgroundFillGradient",
        deserialize = "backgroundFillGradient"
    ))]
    Gradient(crate::types::BackgroundFillGradient),
    /// Describes a freeform gradient fill of a background
    #[serde(rename(
        serialize = "backgroundFillFreeformGradient",
        deserialize = "backgroundFillFreeformGradient"
    ))]
    FreeformGradient(crate::types::BackgroundFillFreeformGradient),
}
