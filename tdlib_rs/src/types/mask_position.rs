#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Position on a photo where a mask is placed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MaskPosition {
    /// Part of the face, relative to which the mask is placed
    pub point: crate::enums::MaskPoint,
    /// Shift by X-axis measured in widths of the mask scaled to the face size, from left to right. (For example, -1.0 will place the mask just to the left of the default mask position)
    pub x_shift: f64,
    /// Shift by Y-axis measured in heights of the mask scaled to the face size, from top to bottom. (For example, 1.0 will place the mask just below the default mask position)
    pub y_shift: f64,
    /// Mask scaling coefficient. (For example, 2.0 means a doubled size)
    pub scale: f64,
}
