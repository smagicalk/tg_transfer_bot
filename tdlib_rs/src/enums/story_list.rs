#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryList {
    /// The list of stories, shown in the main chat list and folder chat lists
    #[serde(rename(serialize = "storyListMain", deserialize = "storyListMain"))]
    Main,
    /// The list of stories, shown in the Arvhive chat list
    #[serde(rename(serialize = "storyListArchive", deserialize = "storyListArchive"))]
    Archive,
}
