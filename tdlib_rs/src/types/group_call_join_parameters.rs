#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes parameters used to join a group call
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GroupCallJoinParameters {
    /// Audio channel synchronization source identifier; received from tgcalls
    pub audio_source_id: i32,
    /// Group call join payload; received from tgcalls
    pub payload: String,
    /// Pass true to join the call with muted microphone
    pub is_muted: bool,
    /// Pass true if the user's video is enabled
    pub is_my_video_enabled: bool,
}
