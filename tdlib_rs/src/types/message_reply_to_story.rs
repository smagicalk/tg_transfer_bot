#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a story replied by a given message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageReplyToStory {
    /// The identifier of the poster of the story
    pub story_poster_chat_id: i64,
    /// The identifier of the story
    pub story_id: i32,
}
