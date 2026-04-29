#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a position of a chat in a chat list
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatPosition {
    /// The chat list
    pub list: crate::enums::ChatList,
    /// A parameter used to determine order of the chat in the chat list. Chats must be sorted by the pair (order, chat.id) in descending order
    #[serde_as(as = "DisplayFromStr")]
    pub order: i64,
    /// True, if the chat is pinned in the chat list
    pub is_pinned: bool,
    /// Source of the chat in the chat list; may be null
    pub source: Option<crate::enums::ChatSource>,
}
