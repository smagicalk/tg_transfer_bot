#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains 0-based match position
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FoundPosition {
    /// The position of the match
    pub position: i32,
}
