#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains identifiers of group call participants
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GroupCallParticipants {
    /// Total number of group call participants
    pub total_count: i32,
    /// Identifiers of the participants
    pub participant_ids: Vec<crate::enums::MessageSender>,
}
