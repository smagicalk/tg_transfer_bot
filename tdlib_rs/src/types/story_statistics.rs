#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A detailed statistics about a story
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StoryStatistics {
    /// A graph containing number of story views and shares
    pub story_interaction_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of story reactions
    pub story_reaction_graph: crate::enums::StatisticalGraph,
}
