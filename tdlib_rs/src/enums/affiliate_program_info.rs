#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AffiliateProgramInfo {
    /// Contains information about an active affiliate program
    #[serde(rename(
        serialize = "affiliateProgramInfo",
        deserialize = "affiliateProgramInfo"
    ))]
    AffiliateProgramInfo(crate::types::AffiliateProgramInfo),
}
