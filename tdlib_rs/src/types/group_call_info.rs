#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about a just created or just joined group call
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GroupCallInfo {
    /// Identifier of the group call
    pub group_call_id: i32,
    /// Join response payload for tgcalls; empty if the call isn't joined
    pub join_payload: String,
}
