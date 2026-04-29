#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AffiliateInfo {
    /// Contains information about an affiliate that received commission from a Telegram Star transaction
    #[serde(rename(serialize = "affiliateInfo", deserialize = "affiliateInfo"))]
    AffiliateInfo(crate::types::AffiliateInfo),
}
