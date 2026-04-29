#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FoundAffiliatePrograms {
    /// Represents a list of found affiliate programs
    #[serde(rename(serialize = "foundAffiliatePrograms", deserialize = "foundAffiliatePrograms"))]
    FoundAffiliatePrograms(crate::types::FoundAffiliatePrograms),
}
