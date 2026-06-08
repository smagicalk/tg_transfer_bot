#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryInteractionType {
    /// A view of the story
    #[serde(rename(
        serialize = "storyInteractionTypeView",
        deserialize = "storyInteractionTypeView"
    ))]
    View(crate::types::StoryInteractionTypeView),
    /// A forward of the story as a message
    #[serde(rename(
        serialize = "storyInteractionTypeForward",
        deserialize = "storyInteractionTypeForward"
    ))]
    Forward(crate::types::StoryInteractionTypeForward),
    /// A repost of the story as a story
    #[serde(rename(
        serialize = "storyInteractionTypeRepost",
        deserialize = "storyInteractionTypeRepost"
    ))]
    Repost(crate::types::StoryInteractionTypeRepost),
}
