#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Some messages were deleted
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateDeleteMessages {
    /// Chat identifier
    pub chat_id: i64,
    /// Identifiers of the deleted messages
    pub message_ids: Vec<i64>,
    /// True, if the messages are permanently deleted by a user (as opposed to just becoming inaccessible)
    pub is_permanent: bool,
    /// True, if the messages are deleted only from the cache and can possibly be retrieved again in the future
    pub from_cache: bool,
}
