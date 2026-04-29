#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A General forum topic has been hidden or unhidden
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageForumTopicIsHiddenToggled {
    /// True, if the topic was hidden; otherwise, the topic was unhidden
    pub is_hidden: bool,
}
