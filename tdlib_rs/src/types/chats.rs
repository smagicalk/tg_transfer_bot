#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a list of chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Chats {
    /// Approximate total number of chats found
    pub total_count: i32,
    /// List of chat identifiers
    pub chat_ids: Vec<i64>,
}
