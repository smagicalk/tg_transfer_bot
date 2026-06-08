#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Point {
    /// A point on a Cartesian plane
    #[serde(rename(serialize = "point", deserialize = "point"))]
    Point(crate::types::Point),
}
