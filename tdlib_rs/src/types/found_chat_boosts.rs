#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of boosts applied to a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FoundChatBoosts {
    /// Total number of boosts applied to the chat
    pub total_count: i32,
    /// List of boosts
    pub boosts: Vec<crate::types::ChatBoost>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}
