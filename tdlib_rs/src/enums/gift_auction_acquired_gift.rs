#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftAuctionAcquiredGift {
    /// Represents a gift that was acquired by the current user on an auction
    #[serde(rename(
        serialize = "giftAuctionAcquiredGift",
        deserialize = "giftAuctionAcquiredGift"
    ))]
    GiftAuctionAcquiredGift(crate::types::GiftAuctionAcquiredGift),
}
