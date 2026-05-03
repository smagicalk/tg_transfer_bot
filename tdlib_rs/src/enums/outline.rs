#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Outline {
    /// Represents outline of an image
    #[serde(rename(serialize = "outline", deserialize = "outline"))]
    Outline(crate::types::Outline),
}
