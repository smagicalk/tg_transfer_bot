#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Information about suggested post of a message was changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageSuggestedPostInfo {
    /// Chat identifier
    pub chat_id: i64,
    /// Message identifier
    pub message_id: i64,
    /// The new information about the suggested post
    pub suggested_post_info: crate::types::SuggestedPostInfo,
}
