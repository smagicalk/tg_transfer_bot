#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The message pinned state was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageIsPinned {
    /// Chat identifier
    pub chat_id: i64,
    /// The message identifier
    pub message_id: i64,
    /// True, if the message is pinned
    pub is_pinned: bool,
}
