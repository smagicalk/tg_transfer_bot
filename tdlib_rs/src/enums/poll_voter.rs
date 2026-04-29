#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PollVoter {
    /// Represents a poll voter
    #[serde(rename(serialize = "pollVoter", deserialize = "pollVoter"))]
    PollVoter(crate::types::PollVoter),
}
