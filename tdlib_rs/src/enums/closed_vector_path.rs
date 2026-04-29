#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ClosedVectorPath {
    /// Represents a closed vector path. The path begins at the end point of the last command. The coordinate system origin is in the upper-left corner
    #[serde(rename(serialize = "closedVectorPath", deserialize = "closedVectorPath"))]
    ClosedVectorPath(crate::types::ClosedVectorPath),
}
