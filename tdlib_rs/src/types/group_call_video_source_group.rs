#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a group of video synchronization source identifiers
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GroupCallVideoSourceGroup {
    /// The semantics of sources, one of "SIM" or "FID"
    pub semantics: String,
    /// The list of synchronization source identifiers
    pub source_ids: Vec<i32>,
}
