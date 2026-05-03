#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a list of affiliate programs that were connected to an affiliate
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ConnectedAffiliatePrograms {
    /// The total number of affiliate programs that were connected to the affiliate
    pub total_count: i32,
    /// The list of connected affiliate programs
    pub programs: Vec<crate::types::ConnectedAffiliateProgram>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}
