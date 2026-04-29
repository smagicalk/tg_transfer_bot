#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallRecentSpeaker {
    /// Describes a recently speaking participant in a group call
    #[serde(rename(serialize = "groupCallRecentSpeaker", deserialize = "groupCallRecentSpeaker"))]
    GroupCallRecentSpeaker(crate::types::GroupCallRecentSpeaker),
}
