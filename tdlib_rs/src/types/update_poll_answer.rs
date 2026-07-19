#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A user changed the answer to a poll; for bots only
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdatePollAnswer {
    /// Unique poll identifier
    #[serde_as(as = "DisplayFromStr")]
    pub poll_id: i64,
    /// Identifier of the message sender that changed the answer to the poll
    pub voter_id: crate::enums::MessageSender,
    /// 0-based identifiers of answer options, chosen by the user
    pub option_ids: Vec<i32>,
}
