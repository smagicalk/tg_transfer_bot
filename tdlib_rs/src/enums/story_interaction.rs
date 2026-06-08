#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryInteraction {
    /// Represents interaction with a story
    #[serde(rename(serialize = "storyInteraction", deserialize = "storyInteraction"))]
    StoryInteraction(crate::types::StoryInteraction),
}
