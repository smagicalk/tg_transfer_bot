#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PublicForwards {
    /// Represents a list of public forwards and reposts as a story of a message or a story
    #[serde(rename(serialize = "publicForwards", deserialize = "publicForwards"))]
    PublicForwards(crate::types::PublicForwards),
}
