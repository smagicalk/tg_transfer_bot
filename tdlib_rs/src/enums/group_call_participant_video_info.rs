#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallParticipantVideoInfo {
    /// Contains information about a group call participant's video channel
    #[serde(rename(
        serialize = "groupCallParticipantVideoInfo",
        deserialize = "groupCallParticipantVideoInfo"
    ))]
    GroupCallParticipantVideoInfo(crate::types::GroupCallParticipantVideoInfo),
}
