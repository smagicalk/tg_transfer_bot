#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a message replied by a given message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageReplyToMessage {
    /// The identifier of the chat to which the message belongs; may be 0 if the replied message is in unknown chat
    pub chat_id: i64,
    /// The identifier of the message; may be 0 if the replied message is in unknown chat
    pub message_id: i64,
    /// Chosen quote from the replied message; may be null if none
    pub quote: Option<crate::types::TextQuote>,
    /// Identifier of the checklist task in the original message that was replied; 0 if none
    pub checklist_task_id: i32,
    /// Information about origin of the message if the message was from another chat or topic; may be null for messages from the same chat
    pub origin: Option<crate::enums::MessageOrigin>,
    /// Point in time (Unix timestamp) when the message was sent if the message was from another chat or topic; 0 for messages from the same chat
    pub origin_send_date: i32,
    /// Media content of the message if the message was from another chat or topic; may be null for messages from the same chat and messages without media.
    /// Can be only one of the following types: messageAnimation, messageAudio, messageChecklist, messageContact, messageDice, messageDocument, messageGame,
    /// messageGiveaway, messageGiveawayWinners, messageInvoice, messageLocation, messagePaidMedia, messagePhoto, messagePoll, messageStakeDice, messageSticker, messageStory,
    /// messageText (for link preview), messageVenue, messageVideo, messageVideoNote, or messageVoiceNote
    pub content: Option<crate::enums::MessageContent>,
}
