#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallParticipant {
    /// Represents a group call participant
    #[serde(rename(
        serialize = "groupCallParticipant",
        deserialize = "groupCallParticipant"
    ))]
    GroupCallParticipant(crate::types::GroupCallParticipant),
}
