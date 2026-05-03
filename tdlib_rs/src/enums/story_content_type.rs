#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryContentType {
    /// A photo story
    #[serde(rename(
        serialize = "storyContentTypePhoto",
        deserialize = "storyContentTypePhoto"
    ))]
    Photo,
    /// A video story
    #[serde(rename(
        serialize = "storyContentTypeVideo",
        deserialize = "storyContentTypeVideo"
    ))]
    Video,
    /// A live story
    #[serde(rename(
        serialize = "storyContentTypeLive",
        deserialize = "storyContentTypeLive"
    ))]
    Live,
    /// A story of unknown content type
    #[serde(rename(
        serialize = "storyContentTypeUnsupported",
        deserialize = "storyContentTypeUnsupported"
    ))]
    Unsupported,
}
