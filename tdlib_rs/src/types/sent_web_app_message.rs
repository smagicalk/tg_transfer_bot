#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Information about the message sent by answerWebAppQuery
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SentWebAppMessage {
    /// Identifier of the sent inline message, if known
    pub inline_message_id: String,
}
