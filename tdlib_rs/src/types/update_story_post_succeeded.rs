#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A story has been successfully posted
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateStoryPostSucceeded {
    /// The posted story
    pub story: crate::types::Story,
    /// The previous temporary story identifier
    pub old_story_id: i32,
}
