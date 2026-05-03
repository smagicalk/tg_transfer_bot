#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FoundPositions {
    /// Contains 0-based positions of matched objects
    #[serde(rename(serialize = "foundPositions", deserialize = "foundPositions"))]
    FoundPositions(crate::types::FoundPositions),
}
