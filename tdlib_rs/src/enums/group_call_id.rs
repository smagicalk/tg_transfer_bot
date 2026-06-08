#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallId {
    /// Contains the group call identifier
    #[serde(rename(serialize = "groupCallId", deserialize = "groupCallId"))]
    GroupCallId(crate::types::GroupCallId),
}
