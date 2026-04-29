#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A price for direct messages was changed in the channel chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageDirectMessagePriceChanged {
    /// True, if direct messages group was enabled for the channel; false otherwise
    pub is_enabled: bool,
    /// The new number of Telegram Stars that must be paid by non-administrator users of the channel chat for each message sent to the direct messages group;
    /// 0 if the direct messages group was disabled or the messages are free
    pub paid_message_star_count: i64,
}
