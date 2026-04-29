#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A message with information about an ended call
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageCall {
    /// Persistent unique call identifier; 0 for calls from other devices, which can't be passed as inputCallFromMessage
    #[serde_as(as = "DisplayFromStr")]
    pub unique_id: i64,
    /// True, if the call was a video call
    pub is_video: bool,
    /// Reason why the call was discarded
    pub discard_reason: crate::enums::CallDiscardReason,
    /// Call duration, in seconds
    pub duration: i32,
}
