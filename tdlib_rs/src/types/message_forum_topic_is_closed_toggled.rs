#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A forum topic has been closed or opened
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageForumTopicIsClosedToggled {
    /// True, if the topic was closed; otherwise, the topic was reopened
    pub is_closed: bool,
}
