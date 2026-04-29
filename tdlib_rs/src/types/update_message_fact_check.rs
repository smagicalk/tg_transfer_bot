#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A fact-check added to a message was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageFactCheck {
    /// Chat identifier
    pub chat_id: i64,
    /// Message identifier
    pub message_id: i64,
    /// The new fact-check
    pub fact_check: crate::types::FactCheck,
}
