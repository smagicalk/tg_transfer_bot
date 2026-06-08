#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftResaleParameters {
    /// Describes parameters of a unique gift available for resale
    #[serde(rename(
        serialize = "giftResaleParameters",
        deserialize = "giftResaleParameters"
    ))]
    GiftResaleParameters(crate::types::GiftResaleParameters),
}
