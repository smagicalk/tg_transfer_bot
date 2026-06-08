#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallStreams {
    /// Represents a list of group call streams
    #[serde(rename(serialize = "groupCallStreams", deserialize = "groupCallStreams"))]
    GroupCallStreams(crate::types::GroupCallStreams),
}
