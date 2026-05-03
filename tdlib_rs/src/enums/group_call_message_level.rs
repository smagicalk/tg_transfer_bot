#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallMessageLevel {
    /// Represents a level of features for a message sent in a live story group call
    #[serde(rename(
        serialize = "groupCallMessageLevel",
        deserialize = "groupCallMessageLevel"
    ))]
    GroupCallMessageLevel(crate::types::GroupCallMessageLevel),
}
