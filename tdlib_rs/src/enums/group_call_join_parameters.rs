#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallJoinParameters {
    /// Describes parameters used to join a group call
    #[serde(rename(serialize = "groupCallJoinParameters", deserialize = "groupCallJoinParameters"))]
    GroupCallJoinParameters(crate::types::GroupCallJoinParameters),
}
