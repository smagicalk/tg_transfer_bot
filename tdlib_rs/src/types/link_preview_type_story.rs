#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a link to a story. Link preview description is unavailable
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeStory {
    /// The identifier of the chat that posted the story
    pub story_poster_chat_id: i64,
    /// Story identifier
    pub story_id: i32,
}
