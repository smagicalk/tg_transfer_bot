#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumLimit {
    /// Contains information about a limit, increased for Premium users
    #[serde(rename(serialize = "premiumLimit", deserialize = "premiumLimit"))]
    PremiumLimit(crate::types::PremiumLimit),
}
