#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FoundStories {
    /// Contains a list of stories found by a search
    #[serde(rename(serialize = "foundStories", deserialize = "foundStories"))]
    FoundStories(crate::types::FoundStories),
}
