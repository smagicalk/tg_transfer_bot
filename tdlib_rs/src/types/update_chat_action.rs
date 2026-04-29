#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A message sender activity in the chat has changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatAction {
    /// Chat identifier
    pub chat_id: i64,
    /// Identifier of the specific topic in which the action was performed; may be null if none
    pub topic_id: Option<crate::enums::MessageTopic>,
    /// Identifier of a message sender performing the action
    pub sender_id: crate::enums::MessageSender,
    /// The action
    pub action: crate::enums::ChatAction,
}
