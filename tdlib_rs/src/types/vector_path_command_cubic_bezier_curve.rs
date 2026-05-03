#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A cubic Bézier curve to a given point
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct VectorPathCommandCubicBezierCurve {
    /// The start control point of the curve
    pub start_control_point: crate::types::Point,
    /// The end control point of the curve
    pub end_control_point: crate::types::Point,
    /// The end point of the curve
    pub end_point: crate::types::Point,
}
