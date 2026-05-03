#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Venue {
    /// Describes a venue
    #[serde(rename(serialize = "venue", deserialize = "venue"))]
    Venue(crate::types::Venue),
}
