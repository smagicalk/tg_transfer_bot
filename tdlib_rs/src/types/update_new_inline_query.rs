#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A new incoming inline query; for bots only
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewInlineQuery {
    /// Unique query identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Identifier of the user who sent the query
    pub sender_user_id: i64,
    /// User location; may be null
    pub user_location: Option<crate::types::Location>,
    /// The type of the chat from which the query originated; may be null if unknown
    pub chat_type: Option<crate::enums::ChatType>,
    /// Text of the query
    pub query: String,
    /// Offset of the first entry to return
    pub offset: String,
}
