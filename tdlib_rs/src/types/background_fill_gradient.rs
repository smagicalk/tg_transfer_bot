#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a gradient fill of a background
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BackgroundFillGradient {
    /// A top color of the background in the RGB format
    pub top_color: i32,
    /// A bottom color of the background in the RGB format
    pub bottom_color: i32,
    /// Clockwise rotation angle of the gradient, in degrees; 0-359. Must always be divisible by 45
    pub rotation_angle: i32,
}
