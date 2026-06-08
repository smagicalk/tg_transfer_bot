#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a ready to send inline message. Use sendInlineQueryResultMessage to send the message
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PreparedInlineMessage {
    /// Unique identifier of the inline query to pass to sendInlineQueryResultMessage
    #[serde_as(as = "DisplayFromStr")]
    pub inline_query_id: i64,
    /// Resulted inline message of the query
    pub result: crate::enums::InlineQueryResult,
    /// Types of the chats to which the message can be sent
    pub chat_types: crate::types::TargetChatTypes,
}
