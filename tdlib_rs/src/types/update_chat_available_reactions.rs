#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The chat available reactions were changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatAvailableReactions {
    /// Chat identifier
    pub chat_id: i64,
    /// The new reactions, available in the chat
    pub available_reactions: crate::enums::ChatAvailableReactions,
}
