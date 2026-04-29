#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a story posted on behalf of the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatStatisticsObjectTypeStory {
    /// Story identifier
    pub story_id: i32,
}
