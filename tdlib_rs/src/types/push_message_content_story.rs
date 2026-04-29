#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A message with a story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentStory {
    /// True, if the user was mentioned in the story
    pub is_mention: bool,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}
