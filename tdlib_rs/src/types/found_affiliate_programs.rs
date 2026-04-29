#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a list of found affiliate programs
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FoundAffiliatePrograms {
    /// The total number of found affiliate programs
    pub total_count: i32,
    /// The list of affiliate programs
    pub programs: Vec<crate::types::FoundAffiliateProgram>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}
