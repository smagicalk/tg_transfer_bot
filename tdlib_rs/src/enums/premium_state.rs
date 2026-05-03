#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumState {
    /// Contains state of Telegram Premium subscription and promotion videos for Premium features
    #[serde(rename(serialize = "premiumState", deserialize = "premiumState"))]
    PremiumState(crate::types::PremiumState),
}
