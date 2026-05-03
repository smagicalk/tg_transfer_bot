#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a video chat, i.e. a group call bound to a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct VideoChat {
    /// Group call identifier of an active video chat; 0 if none. Full information about the video chat can be received through the method getGroupCall
    pub group_call_id: i32,
    /// True, if the video chat has participants
    pub has_participants: bool,
    /// Default group call participant identifier to join the video chat; may be null
    pub default_participant_id: Option<crate::enums::MessageSender>,
}
