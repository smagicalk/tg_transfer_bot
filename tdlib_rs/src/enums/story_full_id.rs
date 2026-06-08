#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryFullId {
    /// Contains identifier of a story along with identifier of the chat that posted it
    #[serde(rename(serialize = "storyFullId", deserialize = "storyFullId"))]
    StoryFullId(crate::types::StoryFullId),
}
