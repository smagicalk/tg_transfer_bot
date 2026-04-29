#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a poll voter
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PollVoter {
    /// The voter identifier
    pub voter_id: crate::enums::MessageSender,
    /// Point in time (Unix timestamp) when the vote was added
    pub date: i32,
}
