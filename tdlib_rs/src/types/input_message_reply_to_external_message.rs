#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a message to be replied that is from a different chat or a forum topic; not supported in secret chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessageReplyToExternalMessage {
    /// The identifier of the chat to which the message to be replied belongs
    pub chat_id: i64,
    /// The identifier of the message to be replied in the specified chat. A message can be replied in another chat or forum topic only if messageProperties.can_be_replied_in_another_chat
    pub message_id: i64,
    /// Quote from the message to be replied; pass null if none
    pub quote: Option<crate::types::InputTextQuote>,
    /// Identifier of the checklist task in the message to be replied; pass 0 to reply to the whole message
    pub checklist_task_id: i32,
}
