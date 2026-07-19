#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A new incoming callback query; for bots only
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewCallbackQuery {
    /// Unique query identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Identifier of the user who sent the query
    pub sender_user_id: i64,
    /// Identifier of the chat where the query was sent
    pub chat_id: i64,
    /// Identifier of the message from which the query originated
    pub message_id: i64,
    /// Identifier that uniquely corresponds to the chat to which the message was sent
    #[serde_as(as = "DisplayFromStr")]
    pub chat_instance: i64,
    /// Query payload
    pub payload: crate::enums::CallbackQueryPayload,
}
