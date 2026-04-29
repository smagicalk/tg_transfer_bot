#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SuggestedPostPrice {
    /// Describes price of a suggested post in Telegram Stars
    #[serde(rename(serialize = "suggestedPostPriceStar", deserialize = "suggestedPostPriceStar"))]
    Star(crate::types::SuggestedPostPriceStar),
    /// Describes price of a suggested post in Toncoins
    #[serde(rename(serialize = "suggestedPostPriceTon", deserialize = "suggestedPostPriceTon"))]
    Ton(crate::types::SuggestedPostPriceTon),
}
