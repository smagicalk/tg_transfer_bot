#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The is_forum setting of a channel was toggled
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventIsForumToggled {
    /// New value of is_forum
    pub is_forum: bool,
}
