#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TextEntity {
    /// Represents a part of the text that needs to be formatted in some unusual way
    #[serde(rename(serialize = "textEntity", deserialize = "textEntity"))]
    TextEntity(crate::types::TextEntity),
}
