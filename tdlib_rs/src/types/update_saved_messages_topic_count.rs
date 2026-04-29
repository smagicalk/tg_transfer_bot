#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Number of Saved Messages topics has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateSavedMessagesTopicCount {
    /// Approximate total number of Saved Messages topics
    pub topic_count: i32,
}
