#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A new video chat was scheduled
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageVideoChatScheduled {
    /// Identifier of the video chat. The video chat can be received through the method getGroupCall
    pub group_call_id: i32,
    /// Point in time (Unix timestamp) when the group call is expected to be started by an administrator
    pub start_date: i32,
}
