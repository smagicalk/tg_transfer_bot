#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryInteractionInfo {
    /// Contains information about interactions with a story
    #[serde(rename(
        serialize = "storyInteractionInfo",
        deserialize = "storyInteractionInfo"
    ))]
    StoryInteractionInfo(crate::types::StoryInteractionInfo),
}
