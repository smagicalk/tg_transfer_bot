#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about a Saved Messages topic
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SavedMessagesTopic {
    /// Unique topic identifier
    pub id: i64,
    /// Type of the topic
    pub r#type: crate::enums::SavedMessagesTopicType,
    /// True, if the topic is pinned
    pub is_pinned: bool,
    /// A parameter used to determine order of the topic in the topic list. Topics must be sorted by the order in descending order
    #[serde_as(as = "DisplayFromStr")]
    pub order: i64,
    /// Last message in the topic; may be null if none or unknown
    pub last_message: Option<crate::types::Message>,
    /// A draft of a message in the topic; may be null if none
    pub draft_message: Option<crate::types::DraftMessage>,
}
