#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The list of active stories posted by a specific chat has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatActiveStories {
    /// The new list of active stories
    pub active_stories: crate::types::ChatActiveStories,
}
