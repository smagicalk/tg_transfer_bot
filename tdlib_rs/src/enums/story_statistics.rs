#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryStatistics {
    /// A detailed statistics about a story
    #[serde(rename(serialize = "storyStatistics", deserialize = "storyStatistics"))]
    StoryStatistics(crate::types::StoryStatistics),
}
