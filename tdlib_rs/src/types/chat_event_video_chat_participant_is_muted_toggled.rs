#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A video chat participant was muted or unmuted
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventVideoChatParticipantIsMutedToggled {
    /// Identifier of the affected group call participant
    pub participant_id: crate::enums::MessageSender,
    /// New value of is_muted
    pub is_muted: bool,
}
