#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a public repost to a story
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PublicForwardStory {
    /// Information about the story
    pub story: crate::types::Story,
}
