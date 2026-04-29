#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A message with a forwarded story. Stories can't be forwarded to secret chats. A story can be forwarded only if story.can_be_forwarded
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessageStory {
    /// Identifier of the chat that posted the story
    pub story_poster_chat_id: i64,
    /// Story identifier
    pub story_id: i32,
}
