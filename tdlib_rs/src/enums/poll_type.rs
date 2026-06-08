#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PollType {
    /// A regular poll
    #[serde(rename(serialize = "pollTypeRegular", deserialize = "pollTypeRegular"))]
    Regular(crate::types::PollTypeRegular),
    /// A poll in quiz mode, which has exactly one correct answer option and can be answered only once
    #[serde(rename(serialize = "pollTypeQuiz", deserialize = "pollTypeQuiz"))]
    Quiz(crate::types::PollTypeQuiz),
}
