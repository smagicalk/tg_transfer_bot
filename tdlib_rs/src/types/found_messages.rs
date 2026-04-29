#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of messages found by a search
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FoundMessages {
    /// Approximate total number of messages found; -1 if unknown
    pub total_count: i32,
    /// List of messages
    pub messages: Vec<crate::types::Message>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}
