#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a poll
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Poll {
    /// Unique poll identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Poll question; 1-300 characters. Only custom emoji entities are allowed
    pub question: crate::types::FormattedText,
    /// List of poll answer options
    pub options: Vec<crate::types::PollOption>,
    /// Total number of voters, participating in the poll
    pub total_voter_count: i32,
    /// Identifiers of recent voters, if the poll is non-anonymous
    pub recent_voter_ids: Vec<crate::enums::MessageSender>,
    /// True, if the poll is anonymous
    pub is_anonymous: bool,
    /// Type of the poll
    pub r#type: crate::enums::PollType,
    /// Amount of time the poll will be active after creation, in seconds
    pub open_period: i32,
    /// Point in time (Unix timestamp) when the poll will automatically be closed
    pub close_date: i32,
    /// True, if the poll is closed
    pub is_closed: bool,
}
