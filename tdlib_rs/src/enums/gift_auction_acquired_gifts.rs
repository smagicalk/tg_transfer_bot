#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftAuctionAcquiredGifts {
    /// Represents a list of gifts that were acquired by the current user on an auction
    #[serde(rename(serialize = "giftAuctionAcquiredGifts", deserialize = "giftAuctionAcquiredGifts"))]
    GiftAuctionAcquiredGifts(crate::types::GiftAuctionAcquiredGifts),
}
