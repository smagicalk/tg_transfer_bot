#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Story {
    /// Represents a story
    #[serde(rename(serialize = "story", deserialize = "story"))]
    Story(crate::types::Story),
}
