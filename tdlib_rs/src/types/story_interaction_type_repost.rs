#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A repost of the story as a story
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StoryInteractionTypeRepost {
    /// The reposted story
    pub story: crate::types::Story,
}
