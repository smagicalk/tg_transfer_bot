#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A detailed statistics about a channel chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatStatisticsChannel {
    /// A period to which the statistics applies
    pub period: crate::types::DateRange,
    /// Number of members in the chat
    pub member_count: crate::types::StatisticalValue,
    /// Mean number of times the recently sent messages were viewed
    pub mean_message_view_count: crate::types::StatisticalValue,
    /// Mean number of times the recently sent messages were shared
    pub mean_message_share_count: crate::types::StatisticalValue,
    /// Mean number of times reactions were added to the recently sent messages
    pub mean_message_reaction_count: crate::types::StatisticalValue,
    /// Mean number of times the recently posted stories were viewed
    pub mean_story_view_count: crate::types::StatisticalValue,
    /// Mean number of times the recently posted stories were shared
    pub mean_story_share_count: crate::types::StatisticalValue,
    /// Mean number of times reactions were added to the recently posted stories
    pub mean_story_reaction_count: crate::types::StatisticalValue,
    /// A percentage of users with enabled notifications for the chat; 0-100
    pub enabled_notifications_percentage: f64,
    /// A graph containing number of members in the chat
    pub member_count_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of members joined and left the chat
    pub join_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of members muted and unmuted the chat
    pub mute_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of message views in a given hour in the last two weeks
    pub view_count_by_hour_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of message views per source
    pub view_count_by_source_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of new member joins per source
    pub join_by_source_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of users viewed chat messages per language
    pub language_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of chat message views and shares
    pub message_interaction_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of reactions on messages
    pub message_reaction_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of story views and shares
    pub story_interaction_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of reactions on stories
    pub story_reaction_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of views of associated with the chat instant views
    pub instant_view_interaction_graph: crate::enums::StatisticalGraph,
    /// Detailed statistics about number of views and shares of recently sent messages and posted stories
    pub recent_interactions: Vec<crate::types::ChatStatisticsInteractionInfo>,
}
