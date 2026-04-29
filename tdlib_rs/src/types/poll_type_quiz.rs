#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A poll in quiz mode, which has exactly one correct answer option and can be answered only once
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PollTypeQuiz {
    /// 0-based identifier of the correct answer option; -1 for a yet unanswered poll
    pub correct_option_id: i32,
    /// Text that is shown when the user chooses an incorrect answer or taps on the lamp icon; 0-200 characters with at most 2 line feeds; empty for a yet unanswered poll
    pub explanation: crate::types::FormattedText,
}
