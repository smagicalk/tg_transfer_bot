#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PollOption {
    /// Describes one answer option of a poll
    #[serde(rename(serialize = "pollOption", deserialize = "pollOption"))]
    PollOption(crate::types::PollOption),
}
