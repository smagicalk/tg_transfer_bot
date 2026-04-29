#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Paid messages were refunded
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessagePaidMessagesRefunded {
    /// The number of refunded messages
    pub message_count: i32,
    /// The number of refunded Telegram Stars
    pub star_count: i64,
}
