#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The user has chosen a result of an inline query; for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewChosenInlineResult {
    /// Identifier of the user who sent the query
    pub sender_user_id: i64,
    /// User location; may be null
    pub user_location: Option<crate::types::Location>,
    /// Text of the query
    pub query: String,
    /// Identifier of the chosen result
    pub result_id: String,
    /// Identifier of the sent inline message, if known
    pub inline_message_id: String,
}
