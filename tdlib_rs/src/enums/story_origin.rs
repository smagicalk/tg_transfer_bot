#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryOrigin {
    /// The original story was a public story that was posted by a known chat
    #[serde(rename(serialize = "storyOriginPublicStory", deserialize = "storyOriginPublicStory"))]
    PublicStory(crate::types::StoryOriginPublicStory),
    /// The original story was posted by an unknown user
    #[serde(rename(serialize = "storyOriginHiddenUser", deserialize = "storyOriginHiddenUser"))]
    HiddenUser(crate::types::StoryOriginHiddenUser),
}
