#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallParticipants {
    /// Contains identifiers of group call participants
    #[serde(rename(serialize = "groupCallParticipants", deserialize = "groupCallParticipants"))]
    GroupCallParticipants(crate::types::GroupCallParticipants),
}
