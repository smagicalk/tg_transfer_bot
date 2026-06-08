#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The chat emoji status was changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventEmojiStatusChanged {
    /// Previous emoji status; may be null if none
    pub old_emoji_status: Option<crate::types::EmojiStatus>,
    /// New emoji status; may be null if none
    pub new_emoji_status: Option<crate::types::EmojiStatus>,
}
