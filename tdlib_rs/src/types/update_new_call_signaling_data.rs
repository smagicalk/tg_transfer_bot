#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// New call signaling data arrived
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewCallSignalingData {
    /// The call identifier
    pub call_id: i32,
    /// The data
    pub data: String,
}
