#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AttributeCraftPersistenceProbability {
    /// Describes chance of the crafted gift to have the backdrop or symbol of one of the original gifts
    #[serde(rename(serialize = "attributeCraftPersistenceProbability", deserialize = "attributeCraftPersistenceProbability"))]
    AttributeCraftPersistenceProbability(crate::types::AttributeCraftPersistenceProbability),
}
