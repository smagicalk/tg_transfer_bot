#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AuctionState {
    /// Contains information about an ongoing or scheduled auction
    #[serde(rename(serialize = "auctionStateActive", deserialize = "auctionStateActive"))]
    Active(crate::types::AuctionStateActive),
    /// Contains information about a finished auction
    #[serde(rename(serialize = "auctionStateFinished", deserialize = "auctionStateFinished"))]
    Finished(crate::types::AuctionStateFinished),
}
