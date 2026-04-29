#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A price for paid messages was changed in the supergroup chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessagePaidMessagePriceChanged {
    /// The new number of Telegram Stars that must be paid by non-administrator users of the supergroup chat for each sent message
    pub paid_message_star_count: i64,
}
