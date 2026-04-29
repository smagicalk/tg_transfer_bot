#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The transaction is a purchase of paid media from a channel by the current user; relevant for regular users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeChannelPaidMediaPurchase {
    /// Identifier of the channel chat that sent the paid media
    pub chat_id: i64,
    /// Identifier of the corresponding message with paid media; may be 0 or an identifier of a deleted message
    pub message_id: i64,
    /// The bought media if the transaction wasn't refunded
    pub media: Vec<crate::enums::PaidMedia>,
}
