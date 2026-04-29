#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A message with a poll
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentPoll {
    /// Poll question
    pub question: String,
    /// True, if the poll is regular and not in quiz mode
    pub is_regular: bool,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}
