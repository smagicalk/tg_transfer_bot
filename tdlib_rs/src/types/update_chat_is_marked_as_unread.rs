#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A chat was marked as unread or was read
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatIsMarkedAsUnread {
    /// Chat identifier
    pub chat_id: i64,
    /// New value of is_marked_as_unread
    pub is_marked_as_unread: bool,
}
