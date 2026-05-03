#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AuctionBid {
    /// Describes a bid in an auction
    #[serde(rename(serialize = "auctionBid", deserialize = "auctionBid"))]
    AuctionBid(crate::types::AuctionBid),
}
