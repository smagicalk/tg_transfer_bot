#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A message was edited. Changes in the message content will come in a separate updateMessageContent
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageEdited {
    /// Chat identifier
    pub chat_id: i64,
    /// Message identifier
    pub message_id: i64,
    /// Point in time (Unix timestamp) when the message was edited
    pub edit_date: i32,
    /// New message reply markup; may be null
    pub reply_markup: Option<crate::enums::ReplyMarkup>,
}
