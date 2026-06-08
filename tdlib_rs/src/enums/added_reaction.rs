#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AddedReaction {
    /// Represents a reaction applied to a message
    #[serde(rename(serialize = "addedReaction", deserialize = "addedReaction"))]
    AddedReaction(crate::types::AddedReaction),
}
