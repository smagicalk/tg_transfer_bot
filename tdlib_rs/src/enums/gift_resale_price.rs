#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftResalePrice {
    /// Describes price of a resold gift in Telegram Stars
    #[serde(rename(serialize = "giftResalePriceStar", deserialize = "giftResalePriceStar"))]
    Star(crate::types::GiftResalePriceStar),
    /// Describes price of a resold gift in Toncoins
    #[serde(rename(serialize = "giftResalePriceTon", deserialize = "giftResalePriceTon"))]
    Ton(crate::types::GiftResalePriceTon),
}
