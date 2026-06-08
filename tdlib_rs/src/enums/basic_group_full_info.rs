#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BasicGroupFullInfo {
    /// Contains full information about a basic group
    #[serde(rename(serialize = "basicGroupFullInfo", deserialize = "basicGroupFullInfo"))]
    BasicGroupFullInfo(crate::types::BasicGroupFullInfo),
}
