#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Minithumbnail {
    /// Thumbnail image of a very poor quality and low resolution
    #[serde(rename(serialize = "minithumbnail", deserialize = "minithumbnail"))]
    Minithumbnail(crate::types::Minithumbnail),
}
