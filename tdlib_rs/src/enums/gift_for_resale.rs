#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftForResale {
    /// Describes a gift available for resale
    #[serde(rename(serialize = "giftForResale", deserialize = "giftForResale"))]
    GiftForResale(crate::types::GiftForResale),
}
