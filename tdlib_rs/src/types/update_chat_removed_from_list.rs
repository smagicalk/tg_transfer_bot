#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A chat was removed from a chat list
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatRemovedFromList {
    /// Chat identifier
    pub chat_id: i64,
    /// The chat list from which the chat was removed
    pub chat_list: crate::enums::ChatList,
}
