#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum RestrictionInfo {
    /// Contains information about restrictions that must be applied to a chat or a message
    #[serde(rename(serialize = "restrictionInfo", deserialize = "restrictionInfo"))]
    RestrictionInfo(crate::types::RestrictionInfo),
}
