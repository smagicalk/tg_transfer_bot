#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AddedReactions {
    /// Represents a list of reactions added to a message
    #[serde(rename(serialize = "addedReactions", deserialize = "addedReactions"))]
    AddedReactions(crate::types::AddedReactions),
}
