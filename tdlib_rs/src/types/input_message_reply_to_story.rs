#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a story to be replied
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessageReplyToStory {
    /// The identifier of the poster of the story. Currently, stories can be replied only in the chat that posted the story; channel stories can't be replied
    pub story_poster_chat_id: i64,
    /// The identifier of the story
    pub story_id: i32,
}
