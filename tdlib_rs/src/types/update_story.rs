#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A story was changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateStory {
    /// The new information about the story
    pub story: crate::types::Story,
}
