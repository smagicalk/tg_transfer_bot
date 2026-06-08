#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessFeatures {
    /// Contains information about features, available to Business user accounts
    #[serde(rename(serialize = "businessFeatures", deserialize = "businessFeatures"))]
    BusinessFeatures(crate::types::BusinessFeatures),
}
