#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumGiftCodeInfo {
    /// Contains information about a Telegram Premium gift code
    #[serde(rename(serialize = "premiumGiftCodeInfo", deserialize = "premiumGiftCodeInfo"))]
    PremiumGiftCodeInfo(crate::types::PremiumGiftCodeInfo),
}
