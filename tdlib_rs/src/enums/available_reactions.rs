#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AvailableReactions {
    /// Represents a list of reactions that can be added to a message
    #[serde(rename(serialize = "availableReactions", deserialize = "availableReactions"))]
    AvailableReactions(crate::types::AvailableReactions),
}
