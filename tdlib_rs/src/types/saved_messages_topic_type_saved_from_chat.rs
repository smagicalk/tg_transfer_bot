#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Topic containing messages forwarded from a specific chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SavedMessagesTopicTypeSavedFromChat {
    /// Identifier of the chat
    pub chat_id: i64,
}
