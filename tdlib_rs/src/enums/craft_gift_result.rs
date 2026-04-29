#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CraftGiftResult {
    /// Crafting was successful
    #[serde(rename(serialize = "craftGiftResultSuccess", deserialize = "craftGiftResultSuccess"))]
    Success(crate::types::CraftGiftResultSuccess),
    /// Crafting isn't possible because one of the gifts can't be used for crafting yet
    #[serde(rename(serialize = "craftGiftResultTooEarly", deserialize = "craftGiftResultTooEarly"))]
    TooEarly(crate::types::CraftGiftResultTooEarly),
    /// Crafting isn't possible because one of the gifts isn't suitable for crafting
    #[serde(rename(serialize = "craftGiftResultInvalidGift", deserialize = "craftGiftResultInvalidGift"))]
    InvalidGift,
    /// Crafting has failed
    #[serde(rename(serialize = "craftGiftResultFail", deserialize = "craftGiftResultFail"))]
    Fail,
}
