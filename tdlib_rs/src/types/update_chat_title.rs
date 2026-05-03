#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The title of a chat was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatTitle {
    /// Chat identifier
    pub chat_id: i64,
    /// The new chat title
    pub title: String,
}
