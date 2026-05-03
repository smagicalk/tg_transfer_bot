#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a list of gifts received by a user or a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReceivedGifts {
    /// The total number of received gifts
    pub total_count: i32,
    /// The list of gifts
    pub gifts: Vec<crate::types::ReceivedGift>,
    /// True, if notifications about new gifts of the owner are enabled
    pub are_notifications_enabled: bool,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}
