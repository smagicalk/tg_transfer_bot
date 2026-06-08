#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// State of a gift auction was updated
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateGiftAuctionState {
    /// New state of the auction
    pub state: crate::types::GiftAuctionState,
}
