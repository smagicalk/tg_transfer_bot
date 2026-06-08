#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Chat emoji status has changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatEmojiStatus {
    /// Chat identifier
    pub chat_id: i64,
    /// The new chat emoji status; may be null
    pub emoji_status: Option<crate::types::EmojiStatus>,
}
