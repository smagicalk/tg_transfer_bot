#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A new incoming callback query from a message sent via a bot; for bots only
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewInlineCallbackQuery {
    /// Unique query identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Identifier of the user who sent the query
    pub sender_user_id: i64,
    /// Identifier of the inline message from which the query originated
    pub inline_message_id: String,
    /// An identifier uniquely corresponding to the chat a message was sent to
    #[serde_as(as = "DisplayFromStr")]
    pub chat_instance: i64,
    /// Query payload
    pub payload: crate::enums::CallbackQueryPayload,
}
