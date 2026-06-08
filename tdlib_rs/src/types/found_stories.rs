#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of stories found by a search
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FoundStories {
    /// Approximate total number of stories found
    pub total_count: i32,
    /// List of stories
    pub stories: Vec<crate::types::Story>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}
