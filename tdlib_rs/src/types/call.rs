#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a call
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Call {
    /// Call identifier, not persistent
    pub id: i32,
    /// Persistent unique call identifier; 0 if isn't assigned yet by the server
    #[serde_as(as = "DisplayFromStr")]
    pub unique_id: i64,
    /// User identifier of the other call participant
    pub user_id: i64,
    /// True, if the call is outgoing
    pub is_outgoing: bool,
    /// True, if the call is a video call
    pub is_video: bool,
    /// Call state
    pub state: crate::enums::CallState,
}
