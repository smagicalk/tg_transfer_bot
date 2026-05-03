#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ActiveStoryState {
    /// The chat has an active live story
    #[serde(rename(
        serialize = "activeStoryStateLive",
        deserialize = "activeStoryStateLive"
    ))]
    Live(crate::types::ActiveStoryStateLive),
    /// The chat has some unread active stories
    #[serde(rename(
        serialize = "activeStoryStateUnread",
        deserialize = "activeStoryStateUnread"
    ))]
    Unread,
    /// The chat has active stories, all of which were read
    #[serde(rename(
        serialize = "activeStoryStateRead",
        deserialize = "activeStoryStateRead"
    ))]
    Read,
}
