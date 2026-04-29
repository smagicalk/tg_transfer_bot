#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Document {
    /// Describes a document of any type
    #[serde(rename(serialize = "document", deserialize = "document"))]
    Document(crate::types::Document),
}
