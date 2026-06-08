#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatActiveStories {
    /// Describes active stories posted by a chat
    #[serde(rename(serialize = "chatActiveStories", deserialize = "chatActiveStories"))]
    ChatActiveStories(crate::types::ChatActiveStories),
}
