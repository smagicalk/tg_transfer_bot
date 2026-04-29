#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains statistics about interactions with a message sent in the chat or a story posted on behalf of the chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatStatisticsInteractionInfo {
    /// Type of the object
    pub object_type: crate::enums::ChatStatisticsObjectType,
    /// Number of times the object was viewed
    pub view_count: i32,
    /// Number of times the object was forwarded
    pub forward_count: i32,
    /// Number of times reactions were added to the object
    pub reaction_count: i32,
}
