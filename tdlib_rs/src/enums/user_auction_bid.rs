#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UserAuctionBid {
    /// Describes a bid of the current user in an auction
    #[serde(rename(serialize = "userAuctionBid", deserialize = "userAuctionBid"))]
    UserAuctionBid(crate::types::UserAuctionBid),
}
