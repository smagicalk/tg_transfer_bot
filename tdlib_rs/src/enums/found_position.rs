#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FoundPosition {
    /// Contains 0-based match position
    #[serde(rename(serialize = "foundPosition", deserialize = "foundPosition"))]
    FoundPosition(crate::types::FoundPosition),
}
