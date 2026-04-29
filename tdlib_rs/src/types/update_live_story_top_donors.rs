#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The list of top donors in live story group call has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateLiveStoryTopDonors {
    /// Identifier of the group call
    pub group_call_id: i32,
    /// New list of live story donors
    pub donors: crate::types::LiveStoryDonors,
}
