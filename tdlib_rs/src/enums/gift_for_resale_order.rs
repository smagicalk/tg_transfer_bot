#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftForResaleOrder {
    /// The gifts will be sorted by their price from the lowest to the highest
    #[serde(rename(serialize = "giftForResaleOrderPrice", deserialize = "giftForResaleOrderPrice"))]
    Price,
    /// The gifts will be sorted by the last date when their price was changed from the newest to the oldest
    #[serde(rename(serialize = "giftForResaleOrderPriceChangeDate", deserialize = "giftForResaleOrderPriceChangeDate"))]
    PriceChangeDate,
    /// The gifts will be sorted by their number from the smallest to the largest
    #[serde(rename(serialize = "giftForResaleOrderNumber", deserialize = "giftForResaleOrderNumber"))]
    Number,
}
