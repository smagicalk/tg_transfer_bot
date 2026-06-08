#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A message with a forwarded story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageStory {
    /// Identifier of the chat that posted the story
    pub story_poster_chat_id: i64,
    /// Story identifier
    pub story_id: i32,
    /// True, if the story was automatically forwarded because of a mention of the user
    pub via_mention: bool,
}
