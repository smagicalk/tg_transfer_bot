#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A straight line to a given point
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct VectorPathCommandLine {
    /// The end point of the straight line
    pub end_point: crate::types::Point,
}
