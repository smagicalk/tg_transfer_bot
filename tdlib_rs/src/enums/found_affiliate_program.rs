#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FoundAffiliateProgram {
    /// Describes a found affiliate program
    #[serde(rename(
        serialize = "foundAffiliateProgram",
        deserialize = "foundAffiliateProgram"
    ))]
    FoundAffiliateProgram(crate::types::FoundAffiliateProgram),
}
