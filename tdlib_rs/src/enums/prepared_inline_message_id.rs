#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PreparedInlineMessageId {
    /// Represents an inline message that can be sent via the bot
    #[serde(rename(serialize = "preparedInlineMessageId", deserialize = "preparedInlineMessageId"))]
    PreparedInlineMessageId(crate::types::PreparedInlineMessageId),
}
