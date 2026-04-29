#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputStoryArea {
    /// Describes a clickable rectangle area on a story media to be added
    #[serde(rename(serialize = "inputStoryArea", deserialize = "inputStoryArea"))]
    InputStoryArea(crate::types::InputStoryArea),
}
