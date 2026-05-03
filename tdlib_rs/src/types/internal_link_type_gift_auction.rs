#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is a link to a gift auction. Call getGiftAuctionState with the given auction identifier to process the link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeGiftAuction {
    /// Unique identifier of the auction
    pub auction_id: String,
}
