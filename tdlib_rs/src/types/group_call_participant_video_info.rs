#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about a group call participant's video channel
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GroupCallParticipantVideoInfo {
    /// List of synchronization source groups of the video
    pub source_groups: Vec<crate::types::GroupCallVideoSourceGroup>,
    /// Video channel endpoint identifier
    pub endpoint_id: String,
    /// True, if the video is paused. This flag needs to be ignored, if new video frames are received
    pub is_paused: bool,
}
