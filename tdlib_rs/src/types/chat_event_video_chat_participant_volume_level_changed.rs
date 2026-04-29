#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A video chat participant volume level was changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventVideoChatParticipantVolumeLevelChanged {
    /// Identifier of the affected group call participant
    pub participant_id: crate::enums::MessageSender,
    /// New value of volume_level; 1-20000 in hundreds of percents
    pub volume_level: i32,
}
