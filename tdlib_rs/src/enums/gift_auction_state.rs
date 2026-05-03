#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftAuctionState {
    /// Represent auction state of a gift
    #[serde(rename(serialize = "giftAuctionState", deserialize = "giftAuctionState"))]
    GiftAuctionState(crate::types::GiftAuctionState),
}
