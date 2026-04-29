#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A detailed statistics about a message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageStatistics {
    /// A graph containing number of message views and shares
    pub message_interaction_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of message reactions
    pub message_reaction_graph: crate::enums::StatisticalGraph,
}
