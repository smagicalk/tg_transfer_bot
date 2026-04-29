#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes one answer option of a poll
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PollOption {
    /// Option text; 1-100 characters. Only custom emoji entities are allowed
    pub text: crate::types::FormattedText,
    /// Number of voters for this option, available only for closed or voted polls
    pub voter_count: i32,
    /// The percentage of votes for this option; 0-100
    pub vote_percentage: i32,
    /// True, if the option was chosen by the user
    pub is_chosen: bool,
    /// True, if the option is being chosen by a pending setPollAnswer request
    pub is_being_chosen: bool,
}
