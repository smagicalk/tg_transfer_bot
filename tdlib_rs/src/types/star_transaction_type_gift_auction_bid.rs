#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The transaction is a bid on a gift auction; relevant for regular users only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeGiftAuctionBid {
    /// Identifier of the user who will receive the gift
    pub owner_id: crate::enums::MessageSender,
    /// The gift
    pub gift: crate::types::Gift,
}
