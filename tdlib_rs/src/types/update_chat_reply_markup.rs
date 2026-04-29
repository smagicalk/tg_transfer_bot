#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The chat reply markup was changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatReplyMarkup {
    /// Chat identifier
    pub chat_id: i64,
    /// The message from which the reply markup must be used; may be null if there is no default reply markup in the chat
    pub reply_markup_message: Option<crate::types::Message>,
}
