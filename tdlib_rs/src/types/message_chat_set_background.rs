#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A new background was set in the chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageChatSetBackground {
    /// Identifier of the message with a previously set same background; 0 if none. Can be an identifier of a deleted message
    pub old_background_message_id: i64,
    /// The new background
    pub background: crate::types::ChatBackground,
    /// True, if the background was set only for self
    pub only_for_self: bool,
}
