#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BasicGroup {
    /// Represents a basic group of 0-200 users (must be upgraded to a supergroup to accommodate more than 200 users)
    #[serde(rename(serialize = "basicGroup", deserialize = "basicGroup"))]
    BasicGroup(crate::types::BasicGroup),
}
