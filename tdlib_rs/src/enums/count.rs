#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Count {
    /// Contains a counter
    #[serde(rename(serialize = "count", deserialize = "count"))]
    Count(crate::types::Count),
}
