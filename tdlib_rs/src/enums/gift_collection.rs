#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftCollection {
    /// Describes collection of gifts
    #[serde(rename(serialize = "giftCollection", deserialize = "giftCollection"))]
    GiftCollection(crate::types::GiftCollection),
}
