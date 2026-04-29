#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputStoryContent {
    /// A photo story
    #[serde(rename(serialize = "inputStoryContentPhoto", deserialize = "inputStoryContentPhoto"))]
    Photo(crate::types::InputStoryContentPhoto),
    /// A video story
    #[serde(rename(serialize = "inputStoryContentVideo", deserialize = "inputStoryContentVideo"))]
    Video(crate::types::InputStoryContentVideo),
}
