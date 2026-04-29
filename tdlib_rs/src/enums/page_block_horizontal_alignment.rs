#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PageBlockHorizontalAlignment {
    /// The content must be left-aligned
    #[serde(rename(serialize = "pageBlockHorizontalAlignmentLeft", deserialize = "pageBlockHorizontalAlignmentLeft"))]
    Left,
    /// The content must be center-aligned
    #[serde(rename(serialize = "pageBlockHorizontalAlignmentCenter", deserialize = "pageBlockHorizontalAlignmentCenter"))]
    Center,
    /// The content must be right-aligned
    #[serde(rename(serialize = "pageBlockHorizontalAlignmentRight", deserialize = "pageBlockHorizontalAlignmentRight"))]
    Right,
}
