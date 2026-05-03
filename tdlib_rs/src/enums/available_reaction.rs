#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AvailableReaction {
    /// Represents an available reaction
    #[serde(rename(serialize = "availableReaction", deserialize = "availableReaction"))]
    AvailableReaction(crate::types::AvailableReaction),
}
