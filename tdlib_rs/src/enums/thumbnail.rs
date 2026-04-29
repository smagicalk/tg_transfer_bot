#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Thumbnail {
    /// Represents a thumbnail
    #[serde(rename(serialize = "thumbnail", deserialize = "thumbnail"))]
    Thumbnail(crate::types::Thumbnail),
}
