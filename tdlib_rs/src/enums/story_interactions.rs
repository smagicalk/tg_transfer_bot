#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryInteractions {
    /// Represents a list of interactions with a story
    #[serde(rename(serialize = "storyInteractions", deserialize = "storyInteractions"))]
    StoryInteractions(crate::types::StoryInteractions),
}
