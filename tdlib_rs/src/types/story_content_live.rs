#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A live story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryContentLive {
    /// Identifier of the corresponding group call. The group call can be received through the method getGroupCall
    pub group_call_id: i32,
    /// True, if the call is an RTMP stream instead of an ordinary group call
    pub is_rtmp_stream: bool,
}
