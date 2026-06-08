#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a recently speaking participant in a group call
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GroupCallRecentSpeaker {
    /// Group call participant identifier
    pub participant_id: crate::enums::MessageSender,
    /// True, is the user has spoken recently
    pub is_speaking: bool,
}
