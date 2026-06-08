#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A point on a Cartesian plane
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Point {
    /// The point's first coordinate
    pub x: f64,
    /// The point's second coordinate
    pub y: f64,
}
