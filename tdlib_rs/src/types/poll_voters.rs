#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a list of poll voters
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PollVoters {
    /// Approximate total number of poll voters found
    pub total_count: i32,
    /// List of poll voters
    pub voters: Vec<crate::types::PollVoter>,
}
