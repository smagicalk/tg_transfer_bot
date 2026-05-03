#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatBackground {
    /// Describes a background set for a specific chat
    #[serde(rename(serialize = "chatBackground", deserialize = "chatBackground"))]
    ChatBackground(crate::types::ChatBackground),
}
