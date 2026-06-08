#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A chat was blocked or unblocked
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatBlockList {
    /// Chat identifier
    pub chat_id: i64,
    /// Block list to which the chat is added; may be null if none
    pub block_list: Option<crate::enums::BlockList>,
}
