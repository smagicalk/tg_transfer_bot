#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Information about a group call participant was changed. The updates are sent only after the group call is received through getGroupCall and only if the call is joined or being joined
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateGroupCallParticipant {
    /// Identifier of the group call
    pub group_call_id: i32,
    /// New data about the participant
    pub participant: crate::types::GroupCallParticipant,
}
