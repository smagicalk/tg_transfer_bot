#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The mute_new_participants setting of a video chat was toggled
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventVideoChatMuteNewParticipantsToggled {
    /// New value of the mute_new_participants setting
    pub mute_new_participants: bool,
}
