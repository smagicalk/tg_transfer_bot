#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a list of public forwards and reposts as a story of a message or a story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PublicForwards {
    /// Approximate total number of messages and stories found
    pub total_count: i32,
    /// List of found public forwards and reposts
    pub forwards: Vec<crate::enums::PublicForward>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}
