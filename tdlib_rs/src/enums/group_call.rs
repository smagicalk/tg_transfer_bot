#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCall {
    /// Describes a group call
    #[serde(rename(serialize = "groupCall", deserialize = "groupCall"))]
    GroupCall(crate::types::GroupCall),
}
