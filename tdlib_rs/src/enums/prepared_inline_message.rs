#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PreparedInlineMessage {
    /// Represents a ready to send inline message. Use sendInlineQueryResultMessage to send the message
    #[serde(rename(serialize = "preparedInlineMessage", deserialize = "preparedInlineMessage"))]
    PreparedInlineMessage(crate::types::PreparedInlineMessage),
}
