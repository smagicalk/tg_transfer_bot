#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarSubscriptionPricing {
    /// Describes subscription plan paid in Telegram Stars
    #[serde(rename(serialize = "starSubscriptionPricing", deserialize = "starSubscriptionPricing"))]
    StarSubscriptionPricing(crate::types::StarSubscriptionPricing),
}
