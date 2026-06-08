#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftAuction {
    /// Describes an auction on which a gift can be purchased
    #[serde(rename(serialize = "giftAuction", deserialize = "giftAuction"))]
    GiftAuction(crate::types::GiftAuction),
}
