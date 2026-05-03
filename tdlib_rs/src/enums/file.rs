#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum File {
    /// Represents a file
    #[serde(rename(serialize = "file", deserialize = "file"))]
    File(crate::types::File),
}
