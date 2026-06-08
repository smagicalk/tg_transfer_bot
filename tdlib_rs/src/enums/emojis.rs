#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Emojis {
    /// Represents a list of emojis
    #[serde(rename(serialize = "emojis", deserialize = "emojis"))]
    Emojis(crate::types::Emojis),
}
