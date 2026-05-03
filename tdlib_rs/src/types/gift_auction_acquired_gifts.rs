#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a list of gifts that were acquired by the current user on an auction
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftAuctionAcquiredGifts {
    /// The list of acquired gifts
    pub gifts: Vec<crate::types::GiftAuctionAcquiredGift>,
}
