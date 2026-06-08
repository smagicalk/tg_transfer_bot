#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains statistics about messages sent by a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatStatisticsMessageSenderInfo {
    /// User identifier
    pub user_id: i64,
    /// Number of sent messages
    pub sent_message_count: i32,
    /// Average number of characters in sent messages; 0 if unknown
    pub average_character_count: i32,
}
