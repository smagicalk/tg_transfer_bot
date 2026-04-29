#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The user can be messaged, but the messages are paid
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CanSendMessageToUserResultUserHasPaidMessages {
    /// Number of Telegram Stars that must be paid by the current user for each sent message to the user
    pub outgoing_paid_message_star_count: i64,
}
