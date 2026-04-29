#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a connection of the bot with a business account
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessConnection {
    /// Unique identifier of the connection
    pub id: String,
    /// Identifier of the business user who created the connection
    pub user_id: i64,
    /// Chat identifier of the private chat with the user
    pub user_chat_id: i64,
    /// Point in time (Unix timestamp) when the connection was established
    pub date: i32,
    /// Rights of the bot; may be null if the connection was disabled
    pub rights: Option<crate::types::BusinessBotRights>,
    /// True, if the connection is enabled; false otherwise
    pub is_enabled: bool,
}
