#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputStoryAreas {
    /// Contains a list of story areas to be added
    #[serde(rename(serialize = "inputStoryAreas", deserialize = "inputStoryAreas"))]
    InputStoryAreas(crate::types::InputStoryAreas),
}
