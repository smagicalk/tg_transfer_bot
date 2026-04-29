#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A chat was added to a chat list
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatAddedToList {
    /// Chat identifier
    pub chat_id: i64,
    /// The chat list to which the chat was added
    pub chat_list: crate::enums::ChatList,
}
