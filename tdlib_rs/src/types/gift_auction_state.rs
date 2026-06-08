#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represent auction state of a gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GiftAuctionState {
    /// The gift
    pub gift: crate::types::Gift,
    /// Auction state of the gift
    pub state: crate::enums::AuctionState,
}
