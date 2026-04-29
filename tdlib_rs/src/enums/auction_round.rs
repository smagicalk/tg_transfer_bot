#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AuctionRound {
    /// Describes a round of an auction
    #[serde(rename(serialize = "auctionRound", deserialize = "auctionRound"))]
    AuctionRound(crate::types::AuctionRound),
}
