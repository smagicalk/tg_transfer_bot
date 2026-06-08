#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// All reactions are available in the chat, excluding the paid reaction and custom reactions in channel chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatAvailableReactionsAll {
    /// The maximum allowed number of reactions per message; 1-11
    pub max_reaction_count: i32,
}
