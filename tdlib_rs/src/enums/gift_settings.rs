#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftSettings {
    /// Contains settings for gift receiving for a user
    #[serde(rename(serialize = "giftSettings", deserialize = "giftSettings"))]
    GiftSettings(crate::types::GiftSettings),
}
