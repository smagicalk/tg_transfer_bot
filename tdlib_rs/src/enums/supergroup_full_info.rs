#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SupergroupFullInfo {
    /// Contains full information about a supergroup or channel
    #[serde(rename(serialize = "supergroupFullInfo", deserialize = "supergroupFullInfo"))]
    SupergroupFullInfo(crate::types::SupergroupFullInfo),
}
