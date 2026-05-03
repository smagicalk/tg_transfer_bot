#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The list of group call participants that can send and receive encrypted call data has changed; for group calls not bound to a chat only
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateGroupCallParticipants {
    /// Identifier of the group call
    pub group_call_id: i32,
    /// New list of group call participant user identifiers. The identifiers may be invalid or the corresponding users may be unknown.
    /// The participants must be shown in the list of group call participants even if there is no information about them
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub participant_user_ids: Vec<i64>,
}
