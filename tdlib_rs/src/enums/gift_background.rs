#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftBackground {
    /// Describes background of a gift
    #[serde(rename(serialize = "giftBackground", deserialize = "giftBackground"))]
    GiftBackground(crate::types::GiftBackground),
}
