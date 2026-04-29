#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// New message was received through a push notification
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NotificationTypeNewPushMessage {
    /// The message identifier. The message will not be available in the chat history, but the identifier can be used in viewMessages, or as a message to be replied in the same chat
    pub message_id: i64,
    /// Identifier of the sender of the message. Corresponding user or chat may be inaccessible
    pub sender_id: crate::enums::MessageSender,
    /// Name of the sender
    pub sender_name: String,
    /// True, if the message is outgoing
    pub is_outgoing: bool,
    /// Push message content
    pub content: crate::enums::PushMessageContent,
}
