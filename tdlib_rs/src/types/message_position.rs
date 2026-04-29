#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about a message in a specific position
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessagePosition {
    /// 0-based message position in the full list of suitable messages
    pub position: i32,
    /// Message identifier
    pub message_id: i64,
    /// Point in time (Unix timestamp) when the message was sent
    pub date: i32,
}
