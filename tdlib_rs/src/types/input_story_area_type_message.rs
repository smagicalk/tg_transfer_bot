#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An area pointing to a message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputStoryAreaTypeMessage {
    /// Identifier of the chat with the message. Currently, the chat must be a supergroup or a channel chat
    pub chat_id: i64,
    /// Identifier of the message. Use messageProperties.can_be_shared_in_story to check whether the message is suitable
    pub message_id: i64,
}
