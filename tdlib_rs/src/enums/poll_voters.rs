#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PollVoters {
    /// Represents a list of poll voters
    #[serde(rename(serialize = "pollVoters", deserialize = "pollVoters"))]
    PollVoters(crate::types::PollVoters),
}
