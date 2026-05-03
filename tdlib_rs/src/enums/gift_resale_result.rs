#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftResaleResult {
    /// Operation was successfully completed
    #[serde(rename(serialize = "giftResaleResultOk", deserialize = "giftResaleResultOk"))]
    Ok(crate::types::GiftResaleResultOk),
    /// Operation has failed, because price has increased. If the price has decreased, then the buying will succeed anyway
    #[serde(rename(
        serialize = "giftResaleResultPriceIncreased",
        deserialize = "giftResaleResultPriceIncreased"
    ))]
    PriceIncreased(crate::types::GiftResaleResultPriceIncreased),
}
