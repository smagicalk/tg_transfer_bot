#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallInfo {
    /// Contains information about a just created or just joined group call
    #[serde(rename(serialize = "groupCallInfo", deserialize = "groupCallInfo"))]
    GroupCallInfo(crate::types::GroupCallInfo),
}
