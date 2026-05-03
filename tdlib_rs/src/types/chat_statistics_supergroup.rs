#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A detailed statistics about a supergroup chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatStatisticsSupergroup {
    /// A period to which the statistics applies
    pub period: crate::types::DateRange,
    /// Number of members in the chat
    pub member_count: crate::types::StatisticalValue,
    /// Number of messages sent to the chat
    pub message_count: crate::types::StatisticalValue,
    /// Number of users who viewed messages in the chat
    pub viewer_count: crate::types::StatisticalValue,
    /// Number of users who sent messages to the chat
    pub sender_count: crate::types::StatisticalValue,
    /// A graph containing number of members in the chat
    pub member_count_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of members joined and left the chat
    pub join_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of new member joins per source
    pub join_by_source_graph: crate::enums::StatisticalGraph,
    /// A graph containing distribution of active users per language
    pub language_graph: crate::enums::StatisticalGraph,
    /// A graph containing distribution of sent messages by content type
    pub message_content_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of different actions in the chat
    pub action_graph: crate::enums::StatisticalGraph,
    /// A graph containing distribution of message views per hour
    pub day_graph: crate::enums::StatisticalGraph,
    /// A graph containing distribution of message views per day of week
    pub week_graph: crate::enums::StatisticalGraph,
    /// List of users sent most messages in the last week
    pub top_senders: Vec<crate::types::ChatStatisticsMessageSenderInfo>,
    /// List of most active administrators in the last week
    pub top_administrators: Vec<crate::types::ChatStatisticsAdministratorActionsInfo>,
    /// List of most active inviters of new members in the last week
    pub top_inviters: Vec<crate::types::ChatStatisticsInviterInfo>,
}
