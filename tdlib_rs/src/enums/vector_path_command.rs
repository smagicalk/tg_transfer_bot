#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum VectorPathCommand {
    /// A straight line to a given point
    #[serde(rename(serialize = "vectorPathCommandLine", deserialize = "vectorPathCommandLine"))]
    Line(crate::types::VectorPathCommandLine),
    /// A cubic Bézier curve to a given point
    #[serde(rename(serialize = "vectorPathCommandCubicBezierCurve", deserialize = "vectorPathCommandCubicBezierCurve"))]
    CubicBezierCurve(crate::types::VectorPathCommandCubicBezierCurve),
}
