#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The list of auctions in which participate the current user has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateActiveGiftAuctions {
    /// New states of the auctions
    pub states: Vec<crate::types::GiftAuctionState>,
}
