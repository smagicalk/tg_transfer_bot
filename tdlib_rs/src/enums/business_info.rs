#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessInfo {
    /// Contains information about a Telegram Business account
    #[serde(rename(serialize = "businessInfo", deserialize = "businessInfo"))]
    BusinessInfo(crate::types::BusinessInfo),
}
