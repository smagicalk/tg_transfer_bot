#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The levels of live story group call messages have changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateGroupCallMessageLevels {
    /// New description of the levels in decreasing order of groupCallMessageLevel.min_star_count
    pub levels: Vec<crate::types::GroupCallMessageLevel>,
}
