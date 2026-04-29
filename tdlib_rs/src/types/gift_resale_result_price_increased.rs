#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Operation has failed, because price has increased. If the price has decreased, then the buying will succeed anyway
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GiftResaleResultPriceIncreased {
    /// New price for the gift
    pub price: crate::enums::GiftResalePrice,
}
