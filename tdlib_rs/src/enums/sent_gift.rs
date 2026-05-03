#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SentGift {
    /// Regular gift
    #[serde(rename(serialize = "sentGiftRegular", deserialize = "sentGiftRegular"))]
    Regular(crate::types::SentGiftRegular),
    /// Upgraded gift
    #[serde(rename(serialize = "sentGiftUpgraded", deserialize = "sentGiftUpgraded"))]
    Upgraded(crate::types::SentGiftUpgraded),
}
