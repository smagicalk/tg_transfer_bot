#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a message to be replied in the same chat and forum topic
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessageReplyToMessage {
    /// The identifier of the message to be replied in the same chat and forum topic. A message can be replied in the same chat and forum topic only if messageProperties.can_be_replied
    pub message_id: i64,
    /// Quote from the message to be replied; pass null if none. Must always be null for replies in secret chats
    pub quote: Option<crate::types::InputTextQuote>,
    /// Identifier of the checklist task in the message to be replied; pass 0 to reply to the whole message
    pub checklist_task_id: i32,
}
